use crate::section::{get_section_info, SECTION_GET_URI_BASE};
use crate::section::schema::{
    EnrollmentStatus,
    PackageEnrollmentStatus,
};
use std::error::Error;
use clap::Parser;
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
    
    #[derive(Debug, Subcommand, PartialEq)]
    pub enum Action {
        Section {
            #[clap(value_parser)]
            subject_code: String,
    
            #[clap(value_parser)]
            course_id: String,
    
            #[clap(value_parser, long)]
            term_code: Option<String>,
        },
        Search {
            #[clap(value_parser)]
            search_key: String,
            
            #[clap(value_parser, short, long)]
            size: Option<usize>,
    
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
    
    if let Action::Section { subject_code, course_id, term_code, .. } = action {
    
        // subject_code, e.g., "266"
        // course_id, e.g., "024798"
        let term_code = term_code.unwrap_or("1234".to_string()); // default spring '23 term code
    
        let url = format!("{}/{}/{}/{}", SECTION_GET_URI_BASE, &term_code, &subject_code, &course_id);
        println!("reading/deserializing json response at {url}..");
        let course_sections = get_section_info(&term_code, &subject_code, &course_id).await?;
    
        // println!("omitted some information; here's the deserialized representation:");
        // println!("{:#?}", &course_sections);
        
        println!("listing important section information for course id {course_id}..");
        for c in &course_sections {
            let PackageEnrollmentStatus { status, .. } = &c.package_enrollment_status;
            let EnrollmentStatus {
                currentlyEnrolled: currently_enrolled,
                capacity,
                waitlistCapacity: waitlist_capacity,
                waitlistCurrentSize: waitlist_current_size,
                ..
            } = &c.enrollment_status;
            
            println!(
                "section #{}:\t{}\t\t({}/{} seats, {}/{} waitlisted)",
                c.id, status, currently_enrolled, capacity, waitlist_current_size, waitlist_capacity
            );
        }
        
    } else {
        eprintln!("The program currently cannot run a general search based on keywords. Use the section subcommand instead.");
    }
    
    Ok(())
}