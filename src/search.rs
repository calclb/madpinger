//! Pertains to information about a specific section.
#![allow(dead_code)]
#![allow(unused_imports)]

use std::fs::File;
use std::io::{stdout, Read, Write};
use std::time::Duration;

use reqwest::header::{
    HeaderMap, HeaderValue, ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, CONNECTION, CONTENT_LENGTH,
    CONTENT_TYPE, USER_AGENT,
};
use reqwest::Client;
use reqwest::Error as ReqwestError;
use serde::Serialize;

use crate::search::schema::{SearchApiPing, SearchedCourse};
use crate::CourseStatusFilters;

const SEARCH_POST_URI_BASE: &str = "https://public.enroll.wisc.edu/api/search/v1";
const OUTPUT_FILE_NAME: &str = "response.json";

pub fn get_payload(
    term_code: &str,
    search: &str,
    page_size: usize,
    filters: CourseStatusFilters,
) -> String {
    let open_str = if filters.open { "OPEN" } else { "" };
    let waitlisted_str = if filters.waitlisted { "WAITLISTED" } else { "" };
    let closed_str = if filters.closed { "CLOSED" } else { "" };

    let s: String = format!(
        r##"{{
    "selectedTerm": "{term_code}",
    "queryString": "{search}",
    "filters": [
    {{
      "has_child": {{
        "type": "enrollmentPackage",
        "query": {{
          "bool": {{
            "must": [
            {{
              "match": {{
                "packageEnrollmentStatus.status": "{open_str} {waitlisted_str} {closed_str}"
              }}
            }},
            {{
              "match": {{
                "published": true
              }}
            }}
            ]
          }}
        }}
      }}
    }}
    ],
    "page": 1,
    "pageSize": {page_size},
    "sortOrder": "SCORE"
  }}"##
    );

    s
}

pub async fn get_search_info(
    client: Client,
    term_code: &str,
    search: &str,
    size: usize,
    filters: CourseStatusFilters,
) -> Result<SearchApiPing, ReqwestError> {
    let payload = get_payload(term_code, search, size, filters);

    // println!("Request body len: {}", payload.len());
    let mut hdrs = HeaderMap::new();
    hdrs.insert(ACCEPT, HeaderValue::from_static("application/json")); // not required
    hdrs.insert(
        ACCEPT_ENCODING,
        HeaderValue::from_static("gzip, deflate, br"),
    ); // not required
    hdrs.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en-US,en;q=0.5")); // not required
    hdrs.insert(CONNECTION, HeaderValue::from_static("keep-alive"));
    hdrs.insert(CONTENT_LENGTH, HeaderValue::from(payload.len()));
    hdrs.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let resp = client
        .post(SEARCH_POST_URI_BASE)
        .headers(hdrs)
        .body(payload)
        .send()
        .await?;

    println!("Response status: {:#?}", resp.status());

    let searched_courses = resp.json::<SearchApiPing>().await?;

    File::create(OUTPUT_FILE_NAME)
        .unwrap()
        .write_all(
            serde_json::to_string(&searched_courses)
                .expect("couldn't convert to string")
                .as_bytes(),
        )
        .expect("failed to write");

    Ok(searched_courses)
}

pub mod schema {
    use crate::section;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct SearchApiPing {
        pub found: usize,
        pub hits: Vec<SearchedCourse>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct SearchedCourse {
        pub term_code: String,
        pub course_id: String,
        pub subject: section::schema::Subject,
        pub catalog_number: String,
        pub approved_for_topics: bool,
        pub topics: Vec<Topic>,
        pub minimum_credits: usize,
        pub maximum_credits: usize,
        pub credit_range: String,
        pub first_taught: Option<String>,
        pub last_taught: Option<String>,
        pub typically_offered: String,
        pub general_ed: Option<ReqGoalAbbrev>,
        pub ethnic_studies: Option<ReqGoalAbbrev>,
        pub breadths: Vec<ReqGoalAbbrev>,
        pub letters_and_science_credits: Option<ReqGoalAbbrev>,
        pub workplace_experience: Option<ReqGoalAbbrev>,
        pub foreign_language: Option<ReqGoalAbbrev>,
        // honors: Option<?>,
        pub levels: Vec<ReqGoalAbbrev>,
        pub open_to_first_year: bool,
        // advisory_prerequisites: Option<AdvisoryPrerequisite>,
        pub enrollment_prerequisites: Option<String>,
        // all_cross_listed_subjets: Vec<?>,
        pub title: String,
        pub description: String,
        pub catalog_print_flag: bool,
        // academic_group_code: Vec<AcademicGroupCode>,
        pub currently_taught: bool,
        // grading_basis: GradingBasis,
        pub repeatable: String,
        // grad_course_work: Option<?>,
        // sustainability: Option<?>,
        // instructor_provided_content: Option<?>,
        // course_requirements: CourseRequirements,
        pub course_designation: String,
        pub course_designation_raw: String,
        pub full_course_designation: String,
        pub full_course_designation_raw: String,
        pub last_updated: u64,
        pub catalog_sort: String,
        pub subject_aggregate: String,
        pub title_suggest: TitleSuggestion,
        #[serde(rename = "matched_queries")]
        pub matched_queries: Option<Vec<String>>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Topic {
        pub short_description: String,
        pub long_description: String,
        pub id: usize,
        pub topic_last_taught: String, // gives a term code
    }

    /// Links a title (seen in `input`) with the course id.
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct TitleSuggestion {
        pub input: Vec<String>,
        pub payload: CourseIdObj,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct CourseIdObj {
        pub course_id: String,
    }

    /// A pair
    /// This type of information seems similar to that displayed by DARS audits.
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ReqGoalAbbrev {
        pub code: String,
        pub description: String,
    }
}
