use std::error::Error;
use std::time::Duration;

use clap::Parser;
use reqwest::header::{HeaderMap, HeaderValue, HOST, USER_AGENT};
use reqwest::Client;

use madpinger::section::schema::{CourseSection, EnrollmentStatus, PackageEnrollmentStatus};
use madpinger::section::{get_section_info, SECTION_GET_URI_BASE};
use madpinger::{search, CourseStatusFilters};
use search::get_search_info;

use crate::config::{Action, Args};

mod section;

const DEFAULT_PAGE_SIZE: usize = 10;
const DEFAULT_TERM_CODE: &str = "1234"; // spring '23

mod config {
    use clap::{command, Parser, Subcommand};

    #[derive(Parser, Debug)]
    #[command(author, version, about, long_about = None)]
    pub struct Args {
        #[clap(subcommand)]
        pub(crate) action: Action,
    }

    #[derive(Debug, Subcommand, PartialEq)]
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
        let term_code = term_code.unwrap_or(DEFAULT_PAGE_SIZE.to_string()); // default spring '23 term code

        let url = format!(
            "{}/{}/{}/{}",
            SECTION_GET_URI_BASE, &term_code, &subject_code, &course_id
        );
        println!("reading/deserializing json response at {url}..");
        let course_sections = get_section_info(&term_code, &subject_code, &course_id).await?;

        // println!("omitted some information; here's the deserialized representation:");
        // println!("{:#?}", &course_sections);

        println!("listing important section information for course id {course_id}..");

        if course_sections.is_empty() {
            eprintln!("No sections found.")
        }

        for c in &course_sections {
            let CourseSection { sections, .. } = &c;
            let PackageEnrollmentStatus { status, .. } = &c.package_enrollment_status;
            let EnrollmentStatus {
                currently_enrolled,
                capacity,
                waitlist_capacity,
                waitlist_current_size,
                ..
            } = &c.enrollment_status;

            // ..some course formatting
            let mut meet_detail_str = String::new();
            for (i, sec) in sections.iter().enumerate() {
                meet_detail_str
                    .push_str(format!("{} {}", sec.assembly_type, sec.section_number).as_str());
                if i != sections.len() - 1 {
                    // if not the last element, separate with comma
                    meet_detail_str.push_str(", ");
                }
            }

            // now print out the course detail
            println!(
                "  {}:  {}  ({}/{} seats, {}/{} waitlisted)",
                meet_detail_str,
                status,
                currently_enrolled,
                capacity,
                waitlist_current_size,
                waitlist_capacity
            );
        }
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

        let term_code = term_code.unwrap_or(DEFAULT_TERM_CODE.to_string()); // default spring '23 term code
        let size = size.unwrap_or(DEFAULT_PAGE_SIZE);
        println!("Searching for '{search_key}'...");
        get_search_info(client, &term_code, &search_key, size, status_filters).await?;
    }
    Ok(())
}
