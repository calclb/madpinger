//! A program that polls the UW-Madison course search and enroll site (https://public.enroll.wisc.edu)

use std::cmp::max;
use std::fmt::{Display, Formatter};

use reqwest::header::{HeaderMap, HeaderValue, HOST, USER_AGENT};
use section::schema::{CourseSection, EnrollmentStatus, PackageEnrollmentStatus};

pub const API_SRC_FILE: &str = "course_sections.csv";
pub const DEFAULT_PAGE_SIZE: usize = 10;
pub const DEFAULT_TERM_CODE: &str = "1242"; // fall '23
pub const DEFAULT_LISTING_SIZE: usize = 5;

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
            currently_enrolled: fb_enrolled, // fallback
            capacity: fb_cap, // fallback
            waitlist_capacity: fb_wcap, // fallback
            waitlist_current_size: fb_wsize, // fallback
            aggregate_currently_enrolled: agg_enrolled,
            aggregate_capacity: agg_cap,
            aggregate_waitlist_capacity: agg_wcap,
            aggregate_waitlist_current_size: agg_wsize,
            ..
        }) = &cs.enrollment_status
        {
            let info_type = match (agg_cap, agg_enrolled, agg_wcap, agg_wsize) {
                (Some(_), Some(_), Some(_), Some(_)) => EnrollInfoType::Aggregate,
                (None, None, None, None) => EnrollInfoType::Fallback,
                _ => EnrollInfoType::Mix,
            };
            
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

            // println!("data: {:?}", &cs);
            // println!("sections {:?}", &sections);
            
            let open_seats = match (agg_cap, agg_enrolled) {
                (Some(cap), Some(enrolled)) => max(0, *cap as isize - *enrolled as isize) as usize,
                _ => max(0, *fb_cap as isize - *fb_enrolled as isize) as usize,
            };
            
            // now print out the course detail
            println!(
                "{} - {}: {} ({} open seats, {}/{} enrolled, {}/{} waitlisted) [{}]",
                course_code,
                meet_detail_str,
                status.pad(),
                open_seats,
                agg_enrolled.map_or(fb_enrolled.to_string(), |v| v.to_string()),
                agg_cap.map_or(fb_cap.to_string(), |v| v.to_string()),
                agg_wsize.map_or(fb_wsize.to_string(), |v| v.to_string()),
                agg_wcap.map_or(fb_wcap.to_string(), |v| v.to_string()),
                info_type
            );
        } else {
            println!("{} - (no sections): {} (n/a)", course_code, status.pad());
        }
    }
}

enum EnrollInfoType {
    Aggregate,
    Fallback,
    Mix,
}

impl Display for EnrollInfoType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EnrollInfoType::Aggregate => write!(f, "AGG"),
            EnrollInfoType::Fallback => write!(f, "FB"),
            EnrollInfoType::Mix => write!(f, "MIX"),
        }
    }
}