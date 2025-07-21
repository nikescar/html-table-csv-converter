use clap::{Command, Arg, ArgMatches};
use clap::ColorChoice;
use html_table_csv_converter::{process_url_to_csv_with_options, CsvOptions};
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
        .arg(
            Arg::new("delimiter")
                .long("delimiter")
                .short('d')
                .value_name("CHAR")
                .help("Change delimiter character (default: ',')")
                .default_value(",")
        )
        .arg(
            Arg::new("quote-fields")
                .long("quote-fields")
                .short('q')
                .value_name("MODE")
                .help("Quote fields mode: never, always, asneeded")
                .default_value("asneeded")
        )
        .arg(
            Arg::new("no-header")
                .long("no-header")
                .action(clap::ArgAction::SetTrue)
                .help("Remove table headers from output")
        )
        .arg(
            Arg::new("quote-columns")
                .long("quote-columns")
                .value_name("COLUMNS")
                .help("Only quote specific columns (comma-separated indices, e.g., '1,3')")
        )
        .arg(
            Arg::new("show-fields")
                .long("show-fields")
                .value_name("COLUMNS")
                .help("Show only specified columns (comma-separated indices, e.g., '1,3,5')")
        )
        .get_matches();

    if let Some(url) = matches.get_one::<String>("url") {
        let options = parse_csv_options(&matches);
        match process_url_to_csv_with_options(url, options) {
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

fn parse_csv_options(matches: &ArgMatches) -> CsvOptions {
    let delimiter = matches.get_one::<String>("delimiter")
        .unwrap()
        .chars()
        .next()
        .unwrap_or(',');

    let quote_mode = match matches.get_one::<String>("quote-fields").unwrap().as_str() {
        "never" => html_table_csv_converter::QuoteMode::Never,
        "always" => html_table_csv_converter::QuoteMode::Always,
        "asneeded" => html_table_csv_converter::QuoteMode::AsNeeded,
        _ => html_table_csv_converter::QuoteMode::AsNeeded,
    };

    let no_header = matches.get_flag("no-header");

    let quote_columns = if let Some(columns_str) = matches.get_one::<String>("quote-columns") {
        columns_str
            .split(',')
            .filter_map(|s| s.trim().parse::<usize>().ok())
            .collect()
    } else {
        Vec::new()
    };

    let show_fields = if let Some(columns_str) = matches.get_one::<String>("show-fields") {
        columns_str
            .split(',')
            .filter_map(|s| s.trim().parse::<usize>().ok())
            .collect()
    } else {
        Vec::new()
    };

    CsvOptions {
        delimiter,
        quote_mode,
        no_header,
        quote_columns,
        show_fields,
    }
}
