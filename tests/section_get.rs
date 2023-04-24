use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Duration;

use madpinger::{default_client_headers, API_SRC_FILE};
use reqwest::Client;
use tokio::time::sleep;

use madpinger::section::{get_section_info, SECTION_GET_URI_BASE};

const BATCH_PAUSE: Duration = Duration::from_secs(10);
const BATCH_REQUEST_SIZE: usize = 100;
const SKIP_LINES: usize = 5000; // don't want the test to go through the entire API every commit

/// Tests that the deserialization process does not throw an error when parsing the API responses from the UW-Madison CS&E site.
/// If an error does occur, it's likely because the schema is still misconfigured.
///
/// Internally, the test reads from `course_sections.csv` to call [`section::get_section_info`]().
///
/// Due to rate-limiting concerns, this function will sometimes run an asynchronous pause.
#[tokio::test]
async fn no_deser_errors_exhaustive() -> Result<(), Box<dyn Error>> {
    let f = File::open(API_SRC_FILE)
        .expect("couldn't open the API file to load necessary request info");
    let br: BufReader<File> = BufReader::new(f);
    let client = Client::builder()
        .default_headers(default_client_headers())
        .cookie_store(true)
        .connect_timeout(Duration::from_secs(10))
        .timeout(Duration::from_secs(10))
        .build()?;

    for (i, l) in br.lines().skip(1).enumerate().skip(SKIP_LINES) {
        // ignore header line
        if let Ok(line) = l {
            let v: Vec<&str> = line.splitn(3, ',').collect();
            let tc = v[0];
            let sc = v[1];
            let cid = v[2];

            println!(
                "hit {}: {}/{}/{}/{}",
                i + 1,
                SECTION_GET_URI_BASE,
                tc,
                sc,
                cid,
            );
            let _ = get_section_info(&client, tc, sc, cid).await?;
        } else {
            println!("(skipped line {}; was malformed)", i);
        }

        if i != 0 && i % BATCH_REQUEST_SIZE == 0 {
            // avoid rate-limiting (or ip blacklist)
            println!("sleeping for {}s", BATCH_PAUSE.as_secs_f64());
            sleep(BATCH_PAUSE).await;
        }
    }

    Ok(())
}

