use clap::{Command, Arg};
use clap::ColorChoice;
use html_table_csv_converter::process_url_to_csv;
use std::process;

fn main() {
    let matches = Command::new("html-table-csv-converter")
        .version("0.1.0")
        .author("HTML Table CSV Converter")
        .about("Downloads HTML from URL and converts tables to CSV")
        .color(ColorChoice::Always)
        .arg(
            Arg::new("url")
                .help("URL to download and extract tables from")
                .required(true)
                .index(1)
        )
        .get_matches();

    if let Some(url) = matches.get_one::<String>("url") {
        match process_url_to_csv(url) {
            Ok(csv_output) => {
                println!("{}", csv_output);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        }
    } else {
        eprintln!("No URL provided");
        process::exit(1);
    }
}
