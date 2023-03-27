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
) -> Result<(), ReqwestError> {
    let payload = get_payload(term_code, search, size, filters);

    println!("Request body len: {}", payload.len());
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

    let text = resp.text().await?;
    println!("Response len: {:#?}", text.len());

    File::create(OUTPUT_FILE_NAME)
        .unwrap()
        .write_all(text.as_bytes())
        .unwrap();
    Ok(())
}
