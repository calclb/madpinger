//! A program that polls the UW-Madison course search and enroll site (https://public.enroll.wisc.edu)
use crate::section::schema::{CourseSection, EnrollmentStatus, PackageEnrollmentStatus};

pub mod section;
pub mod search;

pub fn print_out_sections(course_sections: &Vec<CourseSection>) {
    for c in course_sections {
        let PackageEnrollmentStatus { status, .. } = &c.package_enrollment_status;
        let EnrollmentStatus {
            currently_enrolled,
            capacity,
            waitlist_capacity,
            waitlist_current_size,
            ..
        } = &c.enrollment_status;
        println!(
            "section #{}:\t{}\t\t({}/{} seats, {}/{} waitlisted)",
            c.id, status, currently_enrolled, capacity, waitlist_current_size, waitlist_capacity
        );
    }
}

pub struct CourseStatusFilters {
    pub open: bool,
    pub waitlisted: bool,
    pub closed: bool,
}