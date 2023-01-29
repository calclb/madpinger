#![allow(unused_imports)]
#![allow(dead_code)]

use crate::section::get_section_info;
use crate::section::schema::{
    CatalogRequirementGroups, CourseSection, EnrollmentOptions, EnrollmentStatus, MeetingMap,
    PackageEnrollmentStatus, Status,
};
use std::error::Error;

// use reqwest::header::{ACCEPT, ACCEPT_ENCODING, CONNECTION, CONTENT_LENGTH, CONTENT_TYPE, HeaderMap, HeaderValue, HOST};

mod section;
// mod course;

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

// region
// #[tokio::main]
// async fn main() {
//     let args = Args::parse();
//
//     // println!("key: '{search_key}'");
//     // println!("size: {size}");
//     // println!("filter_str: '{filters}'");
//
//     let mut defaults = HeaderMap::default();
//     defaults.insert(HOST, HeaderValue::from_str("public.enroll.wisc.edu").unwrap());
//     defaults.insert(ACCEPT, HeaderValue::from_str("application/json").unwrap());
//     defaults.insert(CONNECTION, HeaderValue::from_str("keep-alive").unwrap()); // <- request to keep connection open; faster response times in subsequent requests (as socket connection is already established)
//     defaults.insert(ACCEPT_ENCODING, HeaderValue::from_str("gzip, deflate, br").unwrap());
//
//     // Host, Accept, Connection, Accept-Encoding (most are gzip)
//     let client = Client::builder()
//         .default_headers(defaults)
//         .cookie_store(true)
//         .build()
//         .expect("cannot build client");
//
//     get_req(&client).await;
//     // post_req(&client, args).await;
// }
//
// async fn post_req(client: &Client, args: Args) {
//     let Args { search_key, open, waitlisted, closed, size } = args;
//
//     let filters = Filters {
//         open,
//         waitlisted,
//         closed,
//     };
//
//     let size = size.unwrap_or(1);
//
//     let body = format!(r#"
// {{
//     "selectedTerm":"1234",
//     "queryString":"{search_key}",
//     "filters": [
//         {{
//             "has_child": {{
//                 "type":"enrollmentPackage",
//                 "query": {{
//                     "bool": {{
//                         "must": [
//                             {{
//                                 "match": {{
//                                     "packageEnrollmentStatus.status":"{filters}"
//                                 }}
//                             }},
//                             {{
//                                 "match": {{
//                                     "published":true
//                                 }}
//                             }}
//                         ]
//                     }}
//                 }}
//             }}
//         }}
//     ],
//     "page":1,
//     "pageSize":{size},
//     "sortOrder":"SCORE"
// }}"#).trim().to_string();
//
//     println!("{body}");
//
//     // Content-Type (type of content that client is sending in POST request)
//     // Content-Length (UTF8; in bytes)
//     let body = json![body];
//     let byte_len = match body.as_str() {
//         Some(v) => v.len(),
//         None => 0,
//     };
//
//     let mut hdrs = HeaderMap::new();
//     hdrs.insert(CONTENT_TYPE, "application/json".parse().unwrap());
//     hdrs.insert(CONTENT_LENGTH, HeaderValue::from(byte_len));
//
//
//     let req = client.post("https://public.enroll.wisc.edu/api/v1")
//         .headers(hdrs)
//         .json(&body)
//         .send().await;
//
//     println!("{:?}", req);
//
//
//     // let response = req.send().await;
//     // println!("{:?}", response);
// }
//
// async fn get_req(client: &Client) {
//
//     let mut hdrs = HeaderMap::new();
//     hdrs.insert(CONTENT_TYPE, HeaderValue::from_str("application/json").unwrap());
//
//     let response = client.get("https://jsonplaceholder.typicode.com/users")
//         .headers(hdrs)
//         .send()
//         .await
//         .expect("failed to send request");
//
//     println!("{:?}", response);
// }
// endregion