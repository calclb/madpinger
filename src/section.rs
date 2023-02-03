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
<<<<<<< HEAD
        pub(crate) id: String,
        pub(crate) term_code: String,
        pub(crate) subject_code: String,
        pub(crate) catalog_number: String,
        pub(crate) enrollment_class_number: usize,
        pub(crate) package_enrollment_status: PackageEnrollmentStatus,
        pub(crate) credit_range: String,
        pub(crate) class_meetings: Vec<ClassMeeting>,
        pub(crate) instructor_provided_class_details: Option<String>,
        pub(crate) published: bool,
        pub(crate) class_permission_number_enabled: bool,
        pub(crate) sections: Vec<Section>,
        pub(crate) enrollment_options: EnrollmentOptions,
        pub(crate) last_updated: u64,
        pub(crate) enrollment_status: EnrollmentStatus,
        pub(crate) meeting_map: MeetingMap,
        pub(crate) online_only: bool,
        pub(crate) enrollment_requirement_groups: Option<CatalogRequirementGroups>,
        pub(crate) is_asynchronous: bool,
        pub(crate) modes_of_instruction: Vec<String>,
        pub(crate) doc_id: String,
=======
        pub id: String,
        pub term_code: String,
        pub subject_code: String,
        pub catalog_number: String,
        pub enrollment_class_number: usize,
        pub package_enrollment_status: PackageEnrollmentStatus,
        pub credit_range: String,
        pub class_meetings: Vec<ClassMeeting>,
        pub instructor_provided_class_details: Option<String>,
        pub published: bool,
        pub class_permission_number_enabled: bool,
        pub sections: Vec<Section>,
        pub enrollment_options: EnrollmentOptions,
        pub last_updated: u64,
        pub enrollment_status: EnrollmentStatus,
        pub meeting_map: MeetingMap,
        pub online_only: bool,
        pub enrollment_requirement_groups: Option<CatalogRequirementGroups>,
        pub is_asynchronous: bool,
        pub modes_of_instruction: Vec<String>,
        pub doc_id: String,
>>>>>>> main
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct PackageEnrollmentStatus {
<<<<<<< HEAD
        pub(crate) available_seats: Option<usize>,
        pub(crate) waitlist_total: usize,
        pub(crate) status: Status,
=======
        pub available_seats: Option<usize>,
        pub waitlist_total: usize,
        pub status: Status,
>>>>>>> main
    }
    
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ClassMeeting {
<<<<<<< HEAD
        pub(crate) meeting_or_exam_number: String,
        pub(crate) meeting_type: MeetingType,
        pub(crate) meeting_time_start: u64,
        pub(crate) meeting_time_end: u64,
        pub(crate) meeting_days: Option<String>,
        pub(crate) meeting_days_list: Vec<String>,
        pub(crate) building: Option<Building>,
        pub(crate) room: Option<String>,
        pub(crate) exam_date: Option<u64>,
=======
        pub meeting_or_exam_number: String,
        pub meeting_type: MeetingType,
        pub meeting_time_start: u64,
        pub meeting_time_end: u64,
        pub meeting_days: Option<String>,
        pub meeting_days_list: Vec<String>,
        pub building: Option<Building>,
        pub room: Option<String>,
        pub exam_date: Option<u64>,
>>>>>>> main
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    // TODO consider refactoring as enum (oncampus, offcampus: for off campus locations) to eliminate options
    pub struct Building {
<<<<<<< HEAD
        pub(crate) building_code: String,
        pub(crate) building_name: String,
        pub(crate) street_address: Option<String>,
        pub(crate) latitude: Option<f64>,
        pub(crate) longitude: Option<f64>,
        pub(crate) location: Option<Vec<f64>>, // (f64, f64)
=======
        pub building_code: String,
        pub building_name: String,
        pub street_address: Option<String>,
        pub latitude: Option<f64>,
        pub longitude: Option<f64>,
        pub location: Option<Vec<f64>>, // (f64, f64)
>>>>>>> main
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Section {
<<<<<<< HEAD
        pub(crate) class_unique_id: ClassUniqueId,
        pub(crate) published: bool,
        // pub(crate) topic: Option<?>
        pub(crate) start_date: u64,
        pub(crate) end_date: u64,
        pub(crate) active: bool,
        pub(crate) session_code: String, // e,g, "A1"
        pub(crate) subject: Subject,
        pub(crate) catalog_number: String,
        pub(crate) course_id: String,
        #[serde(rename = "type")]
        pub(crate) assembly_type: AssemblyType, // LAB, LEC, DIS
        pub(crate) section_number: String,
        // honors: Option<_>, // TODO what type?
        pub(crate) com_b: bool,
        pub(crate) graded_component: bool,
        pub(crate) instruction_mode: String,
        pub(crate) add_consent: Consent,
        pub(crate) drop_consent: Consent,
        pub(crate) cross_listing: Option<String>,
        pub(crate) class_meetings: Vec<ClassMeeting>,
        // classAttributes: Vec<_>, // TODO what type?
        pub(crate) enrollment_status: EnrollmentStatus,
        pub(crate) footnotes: Vec<String>,
        pub(crate) class_materials: Vec<ClassMaterials>,
        pub(crate) instructors: Vec<PersonAttributes>,
        pub(crate) instructor: Option<Instructor>, // basically a wrapper type of PersonAttributes
=======
        pub class_unique_id: ClassUniqueId,
        pub published: bool,
        // pub topic: Option<?>
        pub start_date: u64,
        pub end_date: u64,
        pub active: bool,
        pub session_code: String, // e,g, "A1"
        pub subject: Subject,
        pub catalog_number: String,
        pub course_id: String,
        #[serde(rename = "type")]
        pub assembly_type: AssemblyType, // LAB, LEC, DIS
        pub section_number: String,
        // honors: Option<_>, // TODO what type?
        pub com_b: bool,
        pub graded_component: bool,
        pub instruction_mode: String,
        pub add_consent: Consent,
        pub drop_consent: Consent,
        pub cross_listing: Option<String>,
        pub class_meetings: Vec<ClassMeeting>,
        // classAttributes: Vec<_>, // TODO what type?
        pub enrollment_status: EnrollmentStatus,
        pub footnotes: Vec<String>,
        pub class_materials: Vec<ClassMaterials>,
        pub instructors: Vec<PersonAttributes>,
        pub instructor: Option<Instructor>, // basically a wrapper type of PersonAttributes
>>>>>>> main
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ClassUniqueId {
<<<<<<< HEAD
        pub(crate) term_code: String,
        pub(crate) class_number: usize,
=======
        pub term_code: String,
        pub class_number: usize,
>>>>>>> main
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Subject {
<<<<<<< HEAD
        pub(crate) term_code: String,
        pub(crate) subject_code: String,
        pub(crate) description: String,
        pub(crate) short_description: String,
        pub(crate) formal_description: String,
        #[serde(rename = "undergraduateCatalogURI")]
        pub(crate) undergraduate_catalog_uri: String,
        #[serde(rename = "departmentURI")]
        pub(crate) department_uri: String,
        pub(crate) udds_funding_source: String,
        pub(crate) school_college: SchoolCollege,
        pub(crate) footnotes: Vec<String>,
        pub(crate) department_owner_academic_org_code: String,
=======
        pub term_code: String,
        pub subject_code: String,
        pub description: String,
        pub short_description: String,
        pub formal_description: String,
        #[serde(rename = "undergraduateCatalogURI")]
        pub undergraduate_catalog_uri: String,
        #[serde(rename = "departmentURI")]
        pub department_uri: String,
        pub udds_funding_source: String,
        pub school_college: SchoolCollege,
        pub footnotes: Vec<String>,
        pub department_owner_academic_org_code: String,
>>>>>>> main
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct SchoolCollege {
<<<<<<< HEAD
        pub(crate) academicOrgCode: String,
        pub(crate) academicGroupCode: String,
        pub(crate) shortDescription: String,
        pub(crate) formalDescription: String,
        pub(crate) uddsCode: Option<String>,
        pub(crate) schoolCollegeURI: String,
=======
        pub academic_org_code: String,
        pub academic_group_code: String,
        pub short_description: String,
        pub formal_description: String,
        pub udds_code: Option<String>,
        #[serde(rename = "schoolCollegeURI")]
        pub school_college_uri: String,
>>>>>>> main
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Consent {
<<<<<<< HEAD
        pub(crate) code: String,
        pub(crate) description: String,
=======
        pub code: String,
        pub description: String,
>>>>>>> main
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct EnrollmentStatus {
        pub class_unique_id: ClassUniqueId,
        pub capacity: usize,
        pub currently_enrolled: usize,
        pub waitlist_capacity: usize,
        pub waitlist_current_size: usize,
        pub open_seats: usize,
        pub open_waitlist_spots: usize,
        pub aggregate_capacity: Option<usize>,
        pub aggregate_currency_enrolled: Option<usize>,
        pub aggregate_waitlist_capacity: Option<usize>,
        pub aggregate_waitlist_current_size: Option<usize>,
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ClassMaterials {
<<<<<<< HEAD
        pub(crate) classUniqueId: ClassUniqueId,
        pub(crate) materialsDefined: bool,
        pub(crate) noMaterialsInstructorMessage: Option<String>,
        pub(crate) sectionNotes: Option<String>,
        pub(crate) lastUpdate: u64,
        pub(crate) relatedUrls: Vec<String>,
        pub(crate) textbooks: Vec<String>,
        pub(crate) otherMaterials: Vec<String>, // TODO are these really Vec<String> types?
=======
        pub class_unique_id: ClassUniqueId,
        pub materials_defined: bool,
        pub no_materials_instructor_message: Option<String>,
        pub section_notes: Option<String>,
        pub last_update: u64,
        pub related_urls: Vec<String>,
        pub textbooks: Vec<String>, // TODO make Textbook struct to represent textbooks and resolve failing test
        pub other_materials: Vec<String>, // TODO are these really Vec<String> types?
>>>>>>> main
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct PersonAttributes {
<<<<<<< HEAD
        pub(crate) emplid: String,
        pub(crate) pvi: String,
        pub(crate) name: InstructorName,
        pub(crate) email: String,
        pub(crate) netid: String,
        pub(crate) campusid: Option<String>,
        pub(crate) office365PrimaryEmail: Option<String>,
=======
        pub emplid: String,
        pub pvi: String,
        pub name: InstructorName,
        pub email: String,
        pub netid: String,
        pub campusid: Option<String>,
        #[serde(rename = "office365PrimaryEmail")]
        pub office365_primary_email: Option<String>,
>>>>>>> main
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct InstructorName {
        pub first: String,
        pub middle: Option<String>,
        pub last: String,
        pub legal_first: Option<String>,
        pub legal_middle: Option<String>,
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Instructor {
        person_attributes: PersonAttributes,
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct EnrollmentOptions {
        pub class_permission_number_needed: bool,
        // relatedClasses: Vec<Class>, // TODO what type is this?
        pub waitlist: bool,
        pub related_class_number: bool,
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct MeetingMap {
        pub monday: bool,
        pub tuesday: bool,
        pub wednesday: bool,
        pub thursday: bool,
        pub friday: bool,
        pub saturday: bool,
        pub sunday: bool,
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct CatalogRequirementGroups {
        pub catalog_requirement_groups: Vec<CatalogRequirement>,
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct CatalogRequirement {
        pub code: String,
        pub description: String,
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

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    pub enum MeetingType {
        Class,
        Exam,
    }
    
    impl Display for MeetingType {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                match *self {
                    Self::Class => "CLASS",
                    Self::Exam => "EXAM",
                }
            )
        }
    }
    
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    pub enum AssemblyType {
        Lec,
        Dis,
        Lab,
    }

    impl Display for AssemblyType {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                match *self {
                    Self::Lec => "LEC",
                    Self::Dis => "DIS",
                    Self::Lab => "LAB",
                }
            )
        }
    }
}
