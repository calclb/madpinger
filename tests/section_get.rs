use madpinger::section::{get_section_info, SECTION_GET_URI_BASE};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Duration;
use tokio::time::sleep;

const BATCH_PAUSE: Duration = Duration::from_secs(5);
const BATCH_REQUEST_SIZE: usize = 20;

/// Tests that the deserialization process does not throw an error when parsing the API responses from the UW-Madison CS&E site.
/// If an error does occur, it's likely because the schema is still misconfigured.
///
/// Internally, the test reads from `course_sections.txt` to call [`section::get_section_info`]().
///
/// Due to rate-limiting concerns, this function will sometimes run an asynchronous pause.
#[tokio::test]
async fn no_errors_exhaustive() -> Result<(), Box<dyn Error>> {
    let f = File::open("./course_sections.txt").expect("could not find `course_section.txt`");
    let br = BufReader::new(f);

    for (i, l) in br.lines().skip(1).enumerate() {
        // ignore header line
        if let Ok(line) = l {
            let v: Vec<&str> = line.splitn(3, "/").collect();
            println!(
                "hit {}: {}/{}",
                i + 1,
                SECTION_GET_URI_BASE,
                line.to_string()
            );
            let _ = get_section_info(v[0], v[1], v[2]).await?;
        }

        if i != 0 && i % BATCH_REQUEST_SIZE == 0 {
            // avoid rate-limiting (or ip blacklist)
            sleep(BATCH_PAUSE).await;
        }
    }

    Ok(())
}
