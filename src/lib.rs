//! A program that polls the UW-Madison course search and enroll site (https://public.enroll.wisc.edu)

use reqwest::header::{HeaderMap, HeaderValue, HOST, USER_AGENT};
use section::schema::{CourseSection, EnrollmentStatus, PackageEnrollmentStatus};
pub const API_SRC_FILE: &str = "course_sections.csv";

pub mod search;
pub mod section;

/// A set of filters representing a search filter
/// for any combination of open, waitlisted, and closed courses.
pub struct CourseStatusFilters {
    pub open: bool,
    pub waitlisted: bool,
    pub closed: bool,
}

/// Returns default client headers for pinging the CS&E API.
pub fn default_client_headers() -> HeaderMap {
    let mut default_headers = HeaderMap::new();
    default_headers.insert(HOST, HeaderValue::from_static("public.enroll.wisc.edu"));
    default_headers.insert(
        USER_AGENT,
        HeaderValue::from_static(
            "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/111.0",
        ),
    );
    default_headers
}

/// Prints out sections of a course.
pub fn report_course_sections(course_sections: &Vec<CourseSection>) {
    if course_sections.is_empty() {
        eprintln!("No sections found.");
    }

    for cs in course_sections {
        let CourseSection {
            sections,
            catalog_number,
            ..
        } = &cs;
        let PackageEnrollmentStatus { status, .. } = &cs.package_enrollment_status;

        let course_code = format!(
            "{} {}",
            &sections
                .get(0)
                .expect("No sections were found!")
                .subject
                .short_description,
            &catalog_number
        );

        if let Some(EnrollmentStatus {
            currently_enrolled,
            capacity,
            waitlist_capacity,
            waitlist_current_size,
            ..
        }) = &cs.enrollment_status
        {
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

            // println!("csidx: {}", csidx);
            // println!("data: {:?}", &cs);
            // println!("sections {:?}", &sections);

            // now print out the course detail
            println!(
                "{} - {}: {} ({}/{} seats, {}/{} waitlisted)",
                course_code,
                meet_detail_str,
                status.pad(),
                currently_enrolled,
                capacity,
                waitlist_current_size,
                waitlist_capacity
            );
        } else {
            println!("{} - (no sections): {} (n/a)", course_code, status.pad());
        }
    }
}
