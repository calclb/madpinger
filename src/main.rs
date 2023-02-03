use madpinger::section::get_section_info;
use madpinger::print_out_sections;
use std::error::Error;

mod config {
    use clap::{command, Parser};

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
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("reading/deserializing response as json..");
    let course_sections = get_section_info("1234", "266", "024798").await?;

    println!("omitted some information; here's the deserialized representation:");
    println!("{:#?}", &course_sections);

    println!("listing important section information..");
    print_out_sections(&course_sections);

    println!("done!");
    Ok(())
}