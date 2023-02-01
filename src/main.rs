use crate::section::get_section_info;
use crate::section::schema::{
    EnrollmentStatus,
    PackageEnrollmentStatus,
};
use std::error::Error;
mod section;

mod config {
    use clap::{command, Parser};
    use std::fmt::{Display, Formatter};

    #[derive(Parser, Debug)]
    #[command(author, version, about, long_about = None)]
    struct Args {
        #[clap(value_parser)]
        search_key: String,

        #[clap(short, long)]
        open: bool,

        #[clap(short, long)]
        waitlisted: bool,

        #[clap(short, long)]
        closed: bool,

        #[clap(value_parser, short, long)]
        size: Option<usize>,
    }

    #[derive(Debug)]
    pub struct Filters {
        pub(crate) open: bool,
        pub(crate) waitlisted: bool,
        pub(crate) closed: bool,
    }

    impl Display for Filters {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let mut s: String = String::new();
            if self.open {
                s.push_str("OPEN");
            }
            if self.waitlisted {
                s.push_str("WAITLISTED");
            }
            if self.closed {
                s.push_str("CLOSED")
            }

            write!(f, "{}", s.trim())
        }
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("reading/deserializing response as json..");
    let course_sections = get_section_info("1234", "266", "024798").await?;

    println!("omitted some information; here's the deserialized representation:");
    println!("{:#?}", &course_sections);

    println!("listing important section information..");
    for c in &course_sections {
        let PackageEnrollmentStatus { status, .. } = &c.packageEnrollmentStatus;
        let EnrollmentStatus {
            currentlyEnrolled,
            capacity,
            waitlistCapacity,
            waitlistCurrentSize,
            ..
        } = &c.enrollmentStatus;
        println!(
            "section #{}:\t{}\t\t({}/{} seats, {}/{} waitlisted)",
            c.id, status, currentlyEnrolled, capacity, waitlistCurrentSize, waitlistCapacity
        );
    }

    println!("done!");
    Ok(())
}