use html_table_csv_converter::process_url_to_csv;

fn main() {
    // Example usage of the HTML table to CSV converter
    let url = "https://en.wikipedia.org/wiki/List_of_countries_by_population";
    
    match process_url_to_csv(url) {
        Ok(csv_output) => {
            println!("CSV output:");
            println!("{}", csv_output);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
