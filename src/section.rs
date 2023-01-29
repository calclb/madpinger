//! Pertains to information about a specific section.
use crate::section::schema::{
    CatalogRequirementGroups, CourseSection, EnrollmentOptions, EnrollmentStatus, MeetingMap,
    PackageEnrollmentStatus, Status,
};
use reqwest::get;
use reqwest::Error as ReqwestError;

const SECTION_GET_URI_BASE: &str =
    "https://public.enroll.wisc.edu/api/search/v1/enrollmentPackages";

/// Retrieves the sections of a course that can be identified with the params.
/// As per [`reqwest`](reqwest)'s docs, note that this **should not be used repeatedly**, as it doesn't maintain a [`Client`](reqwest::Client);
/// instead, it uses reqwest's convenience method [`get()`](reqwest::get).
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

    #[allow(non_snake_case)]
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct CourseSection {
        pub(crate) id: String,
        pub(crate) termCode: String,
        pub(crate) subjectCode: String,
        pub(crate) catalogNumber: String,
        pub(crate) enrollmentClassNumber: usize,
        pub(crate) packageEnrollmentStatus: PackageEnrollmentStatus,
        pub(crate) creditRange: String,
        // classMeetings: Vec<ClassMeeting>,
        // nestedClassMeetings: Vec<ClassMeeting>,
        pub(crate) instructorProvidedClassDetails: Option<String>,
        pub(crate) published: bool,
        pub(crate) classPermissionNumberEnabled: bool,
        // sections: Vec<Subsection>,
        pub(crate) enrollmentOptions: EnrollmentOptions,
        pub(crate) lastUpdated: u64,
        pub(crate) enrollmentStatus: EnrollmentStatus,
        pub(crate) meetingMap: MeetingMap,
        pub(crate) onlineOnly: bool,
        pub(crate) enrollmentRequirementGroups: CatalogRequirementGroups,
        pub(crate) isAsynchronous: bool,
        pub(crate) modesOfInstruction: Vec<String>,
        pub(crate) docId: String,
    }

    #[allow(non_snake_case)]
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct PackageEnrollmentStatus {
        pub(crate) availableSeats: Option<usize>,
        pub(crate) waitlistTotal: usize,
        pub(crate) status: Status, // String
    }

    #[allow(non_snake_case)]
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ClassMeeting {
        pub(crate) meetingOrExamNumber: String,
        pub(crate) meetingType: String,
        pub(crate) meetingTimeStart: u64,
        pub(crate) meetingTimeEnd: u64,
        pub(crate) meetingDays: Option<String>,
        pub(crate) meetingDaysList: Vec<String>,
        pub(crate) building: Option<Building>,
        pub(crate) room: Option<String>,
        pub(crate) examDate: Option<String>, // TODO contained type in Option<..> may be incorrect
        pub(crate) monday: bool,
        pub(crate) tuesday: bool,
        pub(crate) wednesday: bool,
        pub(crate) thursday: bool,
        pub(crate) friday: bool,
        pub(crate) saturday: bool,
        pub(crate) sunday: bool,
        pub(crate) startDate: u64,
        pub(crate) endDate: u64,
    }

    #[allow(non_snake_case)]
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Building {
        pub(crate) buildingCode: String,
        pub(crate) buildingName: String,
        pub(crate) streetAddress: String,
        pub(crate) latitude: f64,
        pub(crate) longitude: f64,
        pub(crate) location: (f64, f64), // represented by [0, 1], aka [Longitude, Latitude]
    }

    #[allow(non_snake_case)]
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Subsection {
        pub(crate) classUniqueId: ClassUniqueId,
        pub(crate) published: bool,
        pub(crate) startDate: u64,
        pub(crate) endDate: u64,
        pub(crate) active: bool,
        pub(crate) sessionCode: String,
        pub(crate) subject: Subject,
        pub(crate) catalogNumber: String,
        pub(crate) courseId: String,
        #[serde(rename = "type")]
        pub(crate) course_type: String,
        pub(crate) sectionNumber: String,
        // honors: Option<_>, // TODO what type?
        pub(crate) comB: bool,
        pub(crate) gradedComponent: bool,
        pub(crate) instructionMode: String,
        pub(crate) addConsent: Consent,
        pub(crate) dropConsent: Consent,
        pub(crate) crossListing: Option<String>, // TODO what type?; check with CS/ECE 252 like courses
        pub(crate) classMeetings: Vec<ClassMeeting>,
        // classAttributes: Vec<_>, // TODO what type?
        pub(crate) enrollmentStatus: EnrollmentStatus,
        pub(crate) footnotes: Vec<String>,
        pub(crate) classMaterials: Vec<ClassMaterials>,
        pub(crate) instructors: Vec<Instructor>,
        pub(crate) instructor: PersonAttributes, // basically a wrapper type of Instructor
    }

    #[allow(non_snake_case)]
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ClassUniqueId {
        pub(crate) termCode: String,
        pub(crate) classNumber: usize,
    }

    #[allow(non_snake_case)]
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Subject {
        pub(crate) termCode: String,
        pub(crate) subjectCode: String,
        pub(crate) description: String,
        pub(crate) shortDescription: String,
        pub(crate) formalDescription: String,
        pub(crate) undergraduateCatalogURI: String,
        pub(crate) graduateCatalogURI: String,
        pub(crate) departmentURI: String,
        pub(crate) uddsFundingSource: String,
        pub(crate) schoolCollege: SchoolCollege,
        pub(crate) footnotes: String,
        pub(crate) departmentOwnerAcademicOrgCode: String,
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
