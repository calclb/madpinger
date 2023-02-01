//! Pertains to information about a specific section.
#![allow(dead_code)]
#![allow(unused_imports)]

use crate::section::schema::{
    CatalogRequirementGroups, CourseSection, EnrollmentOptions, EnrollmentStatus, MeetingMap,
    PackageEnrollmentStatus, Status,
};
use reqwest::get;
use reqwest::Error as ReqwestError;


pub const SECTION_GET_URI_BASE: &str =
    "https://public.enroll.wisc.edu/api/search/v1/enrollmentPackages";

/// Retrieves the sections of a course that can be identified with the params.
/// As per [`reqwest`](reqwest)'s docs, note that this **should not be used repeatedly**, as it doesn't maintain a [`Client`](reqwest::Client);
/// instead, it uses reqwest's convenience method [`get()`](get).
pub async fn get_section_info(
    term_code: &str,
    subject_code: &str,
    course_id: &str,
) -> Result<Vec<CourseSection>, ReqwestError> {
    let url = format!(
        "{}/{}/{}/{}",
        SECTION_GET_URI_BASE, term_code, subject_code, course_id
    );
    let http_response = get(url).await?;
    let course_sections = http_response.json::<Vec<CourseSection>>().await?;
    Ok(course_sections)
}

pub mod schema {
    use serde::Deserialize;
    use std::fmt::{Display, Formatter};

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct CourseSection {
        pub(crate) id: String,
        pub(crate) term_code: String,
        pub(crate) subject_code: String,
        pub(crate) catalog_number: String,
        pub(crate) enrollment_class_number: usize,
        pub(crate) package_enrollment_status: PackageEnrollmentStatus,
        pub(crate) credit_range: String,
        // classMeetings: Vec<ClassMeeting>,
        // nestedClassMeetings: Vec<ClassMeeting>,
        pub(crate) instructor_provided_class_details: Option<String>,
        pub(crate) published: bool,
        pub(crate) class_permission_number_enabled: bool,
        // sections: Vec<Subsection>,
        pub(crate) enrollment_options: EnrollmentOptions,
        pub(crate) last_updated: u64,
        pub(crate) enrollment_status: EnrollmentStatus,
        pub(crate) meeting_map: MeetingMap,
        pub(crate) online_only: bool,
        pub(crate) enrollment_requirement_groups: CatalogRequirementGroups,
        pub(crate) is_asynchronous: bool,
        pub(crate) modes_of_instruction: Vec<String>,
        pub(crate) doc_id: String,
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct PackageEnrollmentStatus {
        pub(crate) available_seats: Option<usize>,
        pub(crate) waitlist_total: usize,
        pub(crate) status: Status, // String
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ClassMeeting {
        pub(crate) meeting_or_exam_number: String,
        pub(crate) meeting_type: String,
        pub(crate) meeting_time_start: u64,
        pub(crate) meeting_time_end: u64,
        pub(crate) meeting_days: Option<String>,
        pub(crate) meeting_days_list: Vec<String>,
        pub(crate) building: Option<Building>,
        pub(crate) room: Option<String>,
        pub(crate) exam_date: Option<String>, // TODO contained type in Option<..> may be incorrect
        pub(crate) monday: bool,
        pub(crate) tuesday: bool,
        pub(crate) wednesday: bool,
        pub(crate) thursday: bool,
        pub(crate) friday: bool,
        pub(crate) saturday: bool,
        pub(crate) sunday: bool,
        pub(crate) start_date: u64,
        pub(crate) end_date: u64,
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Building {
        pub(crate) building_code: String,
        pub(crate) building_name: String,
        pub(crate) street_address: String,
        pub(crate) latitude: f64,
        pub(crate) longitude: f64,
        pub(crate) location: (f64, f64), // represented by [0, 1], aka [Longitude, Latitude]
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Subsection {
        pub(crate) class_unique_id: ClassUniqueId,
        pub(crate) published: bool,
        pub(crate) start_date: u64,
        pub(crate) end_date: u64,
        pub(crate) active: bool,
        pub(crate) session_code: String,
        pub(crate) subject: Subject,
        pub(crate) catalog_number: String,
        pub(crate) course_id: String,
        #[serde(rename = "type")]
        pub(crate) course_type: String,
        pub(crate) section_number: String,
        // honors: Option<_>, // TODO what type?
        pub(crate) com_b: bool,
        pub(crate) graded_component: bool,
        pub(crate) instruction_mode: String,
        pub(crate) add_consent: Consent,
        pub(crate) drop_consent: Consent,
        pub(crate) cross_listing: Option<String>, // TODO what type?; check with CS/ECE 252 like courses
        pub(crate) class_meetings: Vec<ClassMeeting>,
        // classAttributes: Vec<_>, // TODO what type?
        pub(crate) enrollment_status: EnrollmentStatus,
        pub(crate) footnotes: Vec<String>,
        pub(crate) class_materials: Vec<ClassMaterials>,
        pub(crate) instructors: Vec<Instructor>,
        pub(crate) instructor: PersonAttributes, // basically a wrapper type of Instructor
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ClassUniqueId {
        pub(crate) term_code: String,
        pub(crate) class_number: usize,
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Subject {
        pub(crate) term_code: String,
        pub(crate) subject_code: String,
        pub(crate) description: String,
        pub(crate) short_description: String,
        pub(crate) formal_description: String,
        pub(crate) undergraduate_catalog_uri: String,
        pub(crate) graduate_catalog_uri: String,
        pub(crate) department_uri: String,
        pub(crate) udds_funding_source: String,
        pub(crate) school_college: SchoolCollege,
        pub(crate) footnotes: String,
        pub(crate) department_owner_academic_org_code: String,
    }

    #[allow(non_snake_case)]
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct SchoolCollege {
        pub(crate) academicOrgCode: String, // TODO char type?
        pub(crate) academicGroupCode: String,
        pub(crate) shortDescription: String,
        pub(crate) formalDescription: String,
        pub(crate) uddsCode: Option<String>,
        pub(crate) schoolCollegeURI: String,
    }

    #[allow(non_snake_case)]
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Consent {
        pub(crate) code: String, // TODO char type?
        pub(crate) description: String,
    }

    #[allow(non_snake_case)]
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct EnrollmentStatus {
        pub(crate) classUniqueId: ClassUniqueId,
        pub(crate) capacity: usize,
        pub(crate) currentlyEnrolled: usize,
        pub(crate) waitlistCapacity: usize,
        pub(crate) waitlistCurrentSize: usize,
        pub(crate) openSeats: usize,
        pub(crate) openWaitlistSpots: usize,
        pub(crate) aggregateCapacity: Option<usize>,
        pub(crate) aggregateCurrencyEnrolled: Option<usize>,
        pub(crate) aggregateWaitlistCapacity: Option<usize>,
        pub(crate) aggregateWaitlistCurrentSize: Option<usize>,
    }

    #[allow(non_snake_case)]
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ClassMaterials {
        pub(crate) classUniqueId: ClassUniqueId,
        pub(crate) materialsDefined: bool,
        pub(crate) noMaterialsInstructorMessage: String,
        pub(crate) sectionNotes: Option<String>,
        pub(crate) lastUpdate: u64,
        pub(crate) relatedUrls: Vec<String>,
        pub(crate) textbooks: Vec<String>,
        pub(crate) otherMaterials: Vec<String>, // TODO are these really Vec<String> types?
    }

    #[allow(non_snake_case)]
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Instructor {
        pub(crate) emplid: String,
        pub(crate) pvi: String,
        pub(crate) name: InstructorName,
        pub(crate) email: String,
        pub(crate) netid: String,
        pub(crate) campusid: Option<String>,
        pub(crate) office365PrimaryEmail: Option<String>,
    }

    #[allow(non_snake_case)]
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct InstructorName {
        pub(crate) first: String,
        pub(crate) middle: Option<String>,
        pub(crate) last: String,
        pub(crate) legalFirst: Option<String>,
        pub(crate) legalMiddle: Option<String>,
    }

    #[allow(non_camel_case_types)]
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct PersonAttributes(Instructor);

    #[allow(non_snake_case)]
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct EnrollmentOptions {
        pub(crate) classPermissionNumberNeeded: bool,
        // relatedClasses: Vec<Class>, // TODO what type is this?
        pub(crate) waitlist: bool,
        pub(crate) relatedClassNumber: bool,
    }

    #[allow(non_snake_case)]
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct MeetingMap {
        pub(crate) monday: bool,
        pub(crate) tuesday: bool,
        pub(crate) wednesday: bool,
        pub(crate) thursday: bool,
        pub(crate) friday: bool,
        pub(crate) saturday: bool,
        pub(crate) sunday: bool,
    }

    #[allow(non_snake_case)]
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct CatalogRequirementGroups {
        pub(crate) catalogRequirementGroups: Vec<CatalogRequirement>,
    }

    #[allow(non_snake_case)]
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct CatalogRequirement {
        pub(crate) code: String,
        pub(crate) description: String,
        // classAssociationRequirementGroups: Vec<String>, TODO type?
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    pub enum Status {
        Open,
        Waitlisted,
        Closed,
    }

    impl Display for Status {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                match *self {
                    Self::Open => "OPEN",
                    Self::Waitlisted => "WAITLISTED",
                    Self::Closed => "CLOSED",
                }
            )
        }
    }
}
