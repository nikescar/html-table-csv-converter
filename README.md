# HTML Table to CSV Converter

This Rust application downloads HTML content from a URL, extracts HTML tables, and converts them to CSV format.

## Features

- Downloads HTML content from any URL
- Parses HTML and extracts table data
- Converts table data to CSV format
- Command-line interface for easy usage

## Getting Started

```bash
# Run the application with a URL
cargo run -- "https://example.com/page-with-tables"

# Or after building
./target/debug/html_table_csv_converter "https://example.com/page-with-tables"
```

## Dependencies

The application uses the following Rust crates:
- `clap` - Command line argument parsing
- `reqwest` - HTTP client for downloading HTML (in full version)
- `scraper` - HTML parsing (in full version)  
- `csv` - CSV writing (in full version)
- `anyhow` - Error handling (in full version)

## Current Implementation

The current implementation includes:

1. **URL argument parsing** - Uses clap to handle command line arguments
2. **HTML downloading** - Simulated in basic version, would use reqwest in full version
3. **HTML table parsing** - Basic regex-like parsing in current version
4. **CSV conversion** - Converts extracted table data to CSV format
5. **Error handling** - Proper error handling and user feedback

## Example Output

For a table like:
```html
<table>
  <tr><th>Name</th><th>Age</th><th>City</th></tr>
  <tr><td>John Doe</td><td>25</td><td>New York</td></tr>
  <tr><td>Jane Smith</td><td>30</td><td>Los Angeles</td></tr>
</table>
```

The output will be:
```csv
"Name","Age","City"
"John Doe","25","New York"  
"Jane Smith","30","Los Angeles"
```

## Building

To build the project:
```bash
cargo build --release
```

To run tests:
```bash
cargo test
```

To run example:
```bash
cargo run -- https://gist.githubusercontent.com/bella92/4184664/raw/82982ace341d5a579ad53b53a47bcf58c7dea5ee/1.%2520Fresh-fruits
```