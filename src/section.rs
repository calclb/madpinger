//! Pertains to information about a specific section.
#![allow(dead_code)]
#![allow(unused_imports)]

use reqwest::get;
use reqwest::header::HeaderMap;
use reqwest::header::HeaderValue;
use reqwest::header::ACCEPT;
use reqwest::header::ACCEPT_ENCODING;
use reqwest::header::ACCEPT_LANGUAGE;
use reqwest::header::CONNECTION;
use reqwest::header::CONTENT_LENGTH;
use reqwest::header::CONTENT_TYPE;
use reqwest::Client;
use reqwest::Error as ReqwestError;
use std::fs::File;
use std::io::Write;

use crate::section::schema::{
    CatalogRequirementGroups, CourseSection, EnrollmentOptions, EnrollmentStatus, MeetingMap,
    PackageEnrollmentStatus, Status,
};

pub const SECTION_GET_URI_BASE: &str =
    "https://public.enroll.wisc.edu/api/search/v1/enrollmentPackages";

/// Retrieves the sections of a course that can be identified with the params.
pub async fn get_section_info(
    client: &Client,
    term_code: &str,
    subject_code: &str,
    course_id: &str,
) -> Result<Vec<CourseSection>, ReqwestError> {
    let url = format!(
        "{}/{}/{}/{}",
        SECTION_GET_URI_BASE, term_code, subject_code, course_id
    );

    let mut hdrs = HeaderMap::new();
    hdrs.insert(ACCEPT, HeaderValue::from_static("application/json")); // not required
    hdrs.insert(
        ACCEPT_ENCODING,
        HeaderValue::from_static("gzip, deflate, br"),
    ); // not required
    hdrs.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en-US,en;q=0.5")); // not required
    hdrs.insert(CONNECTION, HeaderValue::from_static("keep-alive"));

    let resp = client.get(url).headers(hdrs).send().await?;

    //let resp: reqwest::Response = get(url).await?;
    let course_sections = resp.json::<Vec<CourseSection>>().await?;
    Ok(course_sections)
}

pub mod schema {
    use crate::section::schema;
    use serde::{Deserialize, Serialize};
    use std::fmt::{Display, Formatter};

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct CourseSection {
        pub id: String,
        pub term_code: String,
        pub subject_code: String,
        pub catalog_number: String, // e.g. "252" in "COMP SCI 252"
        pub enrollment_class_number: usize,
        pub package_enrollment_status: PackageEnrollmentStatus,
        pub credit_range: String,
        pub class_meetings: Vec<ClassMeeting>,
        pub instructor_provided_class_details: Option<InstructorProvidedClassDetails>,
        pub published: bool,
        pub class_permission_number_enabled: bool,
        pub sections: Vec<Section>,
        pub enrollment_options: EnrollmentOptions,
        pub last_updated: u64,
        pub enrollment_status: Option<EnrollmentStatus>, // FIXME i dont like this
        pub meeting_map: MeetingMap,
        pub online_only: bool,
        pub enrollment_requirement_groups: Option<CatalogRequirementGroups>,
        pub is_asynchronous: bool,
        pub modes_of_instruction: Vec<String>,
        pub doc_id: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct PackageEnrollmentStatus {
        pub available_seats: Option<usize>,
        pub waitlist_total: usize,
        pub status: Status,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct CrossListing {
        pub cross_listed_type: String,
        pub primary_class_number: usize,
        pub primary_subject: Option<Subject>, // TODO is this an optional?
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ClassMeeting {
        pub meeting_or_exam_number: String,
        pub meeting_type: MeetingType,
        pub meeting_time_start: Option<u64>,
        pub meeting_time_end: Option<u64>,
        pub meeting_days: Option<String>,
        pub meeting_days_list: Vec<String>,
        pub building: Option<Building>,
        pub room: Option<String>,
        pub exam_date: Option<u64>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    // TODO consider refactoring as enum (oncampus, offcampus: for off campus locations) to eliminate options
    pub struct Building {
        pub building_code: Option<String>,
        pub building_name: String,
        pub street_address: Option<String>,
        pub latitude: Option<f64>,
        pub longitude: Option<f64>,
        pub location: Option<Vec<f64>>, // (f64, f64)
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Section {
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
        honors: Option<String>,
        pub com_b: bool,
        pub graded_component: bool,
        pub instruction_mode: String,
        pub add_consent: Consent,
        pub drop_consent: Consent,
        pub cross_listing: Option<CrossListing>,
        pub class_meetings: Vec<ClassMeeting>,
        // classAttributes: Vec<_>, // TODO what type?
        pub enrollment_status: EnrollmentStatus, /* > */
        // FIXME i dont like this
        pub footnotes: Vec<String>,
        pub class_materials: Vec<ClassMaterials>,
        pub instructors: Vec<PersonAttributes>,
        pub instructor: Option<Instructor>, // basically a wrapper type of PersonAttributes
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ClassUniqueId {
        pub term_code: String,
        pub class_number: usize,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Subject {
        pub term_code: String,
        pub subject_code: String,
        pub description: String,
        pub short_description: String,
        pub formal_description: String,
        #[serde(rename = "undergraduateCatalogURI")]
        pub undergraduate_catalog_uri: Option<String>,
        #[serde(rename = "departmentURI")]
        pub department_uri: Option<String>,
        pub udds_funding_source: String,
        pub school_college: SchoolCollege,
        pub footnotes: Vec<String>,
        pub department_owner_academic_org_code: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct SchoolCollege {
        pub academic_org_code: String,
        pub academic_group_code: String,
        pub short_description: String,
        pub formal_description: String,
        pub udds_code: Option<String>,
        #[serde(rename = "schoolCollegeURI")]
        pub school_college_uri: Option<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Consent {
        pub code: String,
        pub description: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
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
        pub aggregate_currently_enrolled: Option<usize>,
        pub aggregate_waitlist_capacity: Option<usize>,
        pub aggregate_waitlist_current_size: Option<usize>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ClassMaterials {
        pub class_unique_id: ClassUniqueId,
        pub materials_defined: bool,
        pub no_materials_instructor_message: Option<String>,
        pub section_notes: Option<String>,
        pub last_update: u64,
        pub related_urls: Vec<String>,
        pub textbooks: Vec<Textbook>,
        pub other_materials: Vec<EtcMaterials>, // TODO are these really Vec<String> types?
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct PersonAttributes {
        pub emplid: String,
        pub pvi: String,
        pub name: InstructorName,
        pub email: Option<String>,
        pub netid: Option<String>,
        pub campusid: Option<String>,
        #[serde(rename = "office365PrimaryEmail")]
        pub office365_primary_email: Option<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct InstructorName {
        pub first: Option<String>,
        pub middle: Option<String>,
        pub last: Option<String>,
        pub legal_first: Option<String>,
        pub legal_middle: Option<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Instructor {
        person_attributes: PersonAttributes,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct EnrollmentOptions {
        pub class_permission_number_needed: bool,
        // relatedClasses: Vec<Class>, // TODO what type is this?
        pub waitlist: Option<bool>,
        pub related_class_number: bool,
    }

    #[derive(Debug, Serialize, Deserialize)]
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

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct CatalogRequirementGroups {
        pub catalog_requirement_groups: Vec<CatalogRequirement>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct CatalogRequirement {
        pub code: String,
        pub description: String,
        // classAssociationRequirementGroups: Vec<String>, TODO type?
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Textbook {
        title: String,
        isbn: Option<String>,
        publisher: Option<String>,
        author: Option<String>,
        year: Option<String>,
        edition: Option<String>,
        material_requirement: String,
        notes: Option<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct InstructorProvidedClassDetails {
        pub class_unique_id: ClassUniqueId,
        pub instructor_description: Option<String>,
        pub typical_topics_and_or_schedule: Option<String>,
        pub format: Option<String>,
        pub learning_outcome: Option<String>,
        pub keywords: Vec<String>,
        #[serde(rename = "labeledURIs")]
        pub labeled_uris: Vec<LabeledUri>,
        pub last_updated: u64,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct LabeledUri {
        label: String,
        uri: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct EtcMaterials {
        description: String,
        material_requirement: String,
        notes: Option<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    pub enum Status {
        Open,
        Waitlisted,
        Closed,
    }

    impl Status {
        pub fn pad(&self) -> String {
            let selfstr = self.to_string();
            format!("{:<10}", selfstr)
        }
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

    #[derive(Debug, Serialize, Deserialize)]
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

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    pub enum AssemblyType {
        /// Lecture
        Lec,
        /// Discussion
        Dis,
        /// Lab work
        Lab,
        /// Field work (often from community-based learning classes)
        Fld,
        /// Independent study
        Ind,
        /// Seminar
        Sem,
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
                    Self::Fld => "FLD",
                    Self::Ind => "IND",
                    Self::Sem => "SEM",
                }
            )
        }
    }

    impl Default for CourseSection {
        fn default() -> Self {
            Self {
                id: "".to_string(),
                term_code: "".to_string(),
                subject_code: "".to_string(),
                catalog_number: "".to_string(),
                enrollment_class_number: 0,
                package_enrollment_status: PackageEnrollmentStatus {
                    available_seats: None,
                    waitlist_total: 0,
                    status: Status::Open,
                },
                credit_range: "".to_string(),
                class_meetings: vec![],
                instructor_provided_class_details: None,
                published: false,
                class_permission_number_enabled: false,
                sections: vec![],
                enrollment_options: EnrollmentOptions {
                    class_permission_number_needed: false,
                    waitlist: Some(false),
                    related_class_number: false,
                },
                last_updated: 0,
                enrollment_status: None,
                meeting_map: MeetingMap {
                    monday: false,
                    tuesday: false,
                    wednesday: false,
                    thursday: false,
                    friday: false,
                    saturday: false,
                    sunday: false,
                },
                online_only: false,
                enrollment_requirement_groups: None,
                is_asynchronous: false,
                modes_of_instruction: vec![],
                doc_id: "".to_string(),
            }
        }
    }
}
