use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::time::Duration;

use clap::Parser;
use reqwest::header::{HeaderMap, HeaderValue, HOST, USER_AGENT};
use reqwest::Client;

use madpinger::search::schema::SearchedCourse;
use madpinger::section::{get_section_info, SECTION_GET_URI_BASE};
use madpinger::{report_course_sections, search, CourseStatusFilters, DEFAULT_PAGE_SIZE, DEFAULT_TERM_CODE, DEFAULT_LISTING_SIZE};
use search::get_search_info;

use crate::config::{Action, Args};

mod section;


mod config {
    use clap::{command, Parser, Subcommand};

    #[derive(Parser, Debug)]
    #[command(author, version, about, long_about = None)]
    pub struct Args {
        #[clap(subcommand)]
        pub(crate) action: Action,
    }

    #[derive(Debug, Subcommand, PartialEq, Eq)]
    pub enum Action {
        Section {
            #[clap(value_parser)]
            subject_code: String,

            #[clap(value_parser)]
            course_id: String,

            #[clap(short, long)]
            term_code: Option<String>,
        },
        Search {
            #[clap(value_parser)]
            search_key: String,

            #[clap(value_parser, short, long)]
            size: Option<usize>,

            #[clap(short, long)]
            term_code: Option<String>,

            #[clap(short, long)]
            open: bool,

            #[clap(short, long)]
            waitlisted: bool,

            #[clap(short, long)]
            closed: bool,
        },
        Listing {
            #[clap(value_parser, short, long)]
            line_start: Option<usize>,

            #[clap(value_parser, short, long)]
            size: Option<usize>,

            #[clap(short, long)]
            print: bool,
        },
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let Args { action, .. } = Args::parse();

    let mut default_headers = HeaderMap::new();
    default_headers.insert(HOST, HeaderValue::from_static("public.enroll.wisc.edu"));
    default_headers.insert(
        USER_AGENT,
        HeaderValue::from_static(
            "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/111.0",
        ),
    );

    let client = Client::builder()
        .default_headers(default_headers)
        .cookie_store(true)
        .connect_timeout(Duration::from_secs(10))
        .timeout(Duration::from_secs(10))
        .build()?;

    if let Action::Section {
        subject_code, // e.g. "266"
        course_id,    // e.g. "024798"
        term_code,    // e.g. "1234" or "1424"
        ..
    } = action
    {
        let term_code = term_code.unwrap_or_else(|| DEFAULT_TERM_CODE.to_string()); // default spring '23 term code

        let url = format!(
            "{}/{}/{}/{}",
            SECTION_GET_URI_BASE, &term_code, &subject_code, &course_id
        );
        println!("reading/deserializing json response at {url}..");
        let course_sections =
            get_section_info(&client, &term_code, &subject_code, &course_id).await?;

        println!("listing important section information for course id {course_id}..");
        report_course_sections(&course_sections);
    } else if let Action::Search {
        search_key,
        size,
        term_code,
        open,
        waitlisted,
        closed,
    } = action
    {
        let status_filters = CourseStatusFilters {
            open,
            waitlisted,
            closed,
        };

        let term_code = term_code.unwrap_or_else(|| DEFAULT_TERM_CODE.to_string()); // default spring '23 term code
        let size = size.unwrap_or(DEFAULT_PAGE_SIZE);
        println!("Searching for '{search_key}'...");
        let api_ping =
            get_search_info(client, &term_code, &search_key, size, status_filters).await?;

        let num_hits = &api_ping.found;
        let hits = api_ping.hits;

        println!("found {} hits", num_hits);
        let mut f: File = File::create("search_results.csv")?;
        f.write_all(b"term_code,subject_code,course_id\n")?;
        for sc in &hits {
            let SearchedCourse {
                course_designation,
                course_id,
                title,
                subject,
                ..
            } = sc;

            println!(
                "cid: {:<10} sc: {:<3} - {:<15} - {}",
                course_id, subject.subject_code, course_designation, title
            );
            f.write_all(
                format!(
                    "{},{},{}\n",
                    subject.term_code, subject.subject_code, course_id
                )
                .as_bytes(),
            )?;
        }
    } else if let Action::Listing {
        line_start,
        size,
        print,
    } = action
    {
        let f: File =
            File::open("course_sections.csv").expect("could not open `course_sections.csv`");
        let br = BufReader::new(f);

        let line_start = line_start.unwrap_or(0);
        let size = size.unwrap_or(DEFAULT_LISTING_SIZE);

        for (i, l) in br.lines().skip(line_start).enumerate().skip(1) {
            if i > size {
                break;
            }

            if let Ok(line) = l {
                let v: Vec<&str> = line.splitn(3, ',').collect();
                let tc = v[0];
                let sc = v[1];
                let cid = v[2];

                let url = format!("{}/{}/{}/{}", SECTION_GET_URI_BASE, tc, sc, cid);
                println!("{i}: reading/deserializing json response at {url}..");
                let course_sections = get_section_info(&client, tc, sc, cid).await?;

                if print {
                    report_course_sections(&course_sections);
                    println!();
                }
            } else {
                println!("(skipped line {}; was malformed)", i);
            }
        }
    }
    Ok(())
}
