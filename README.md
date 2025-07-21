# HTML Table to CSV Converter

This Rust application downloads HTML content from a URL, extracts HTML tables, and converts them to CSV format.

<details markdown>

<summary> Features </summary>

## Features

- Downloads HTML content from any URL
- Parses HTML and extracts table data
- Converts table data to CSV format with customizable options
- Command-line interface for easy usage
- Configurable CSV delimiter
- Flexible field quoting options
- Header removal option
- Column-specific quoting
- Column filtering (show only specified columns)

## Getting Started

```bash
# Basic usage - download and convert tables from a URL
cargo run -- "https://example.com/page-with-tables"

# Use pipe delimiter instead of comma
cargo run -- -d "|" "https://example.com/page-with-tables"

# Always quote all fields
cargo run -- --quote-fields always "https://example.com/page-with-tables"

# Remove headers from output
cargo run -- --no-header "https://example.com/page-with-tables"

# Quote only specific columns (1-based indexing)
cargo run -- --quote-columns "1,3" "https://example.com/page-with-tables"

# Show only specific columns
cargo run -- --show-fields "1,3" "https://example.com/page-with-tables"

# Combine options
cargo run -- -d ";" --quote-fields never --no-header --show-fields "2,4" "https://example.com/page-with-tables"

# Or after building
./target/release/html-table-csv-converter --help
```

## Dependencies

The application uses the following Rust crates:
- `clap` - Command line argument parsing
- `ureq` - HTTP client for downloading HTML with rustls TLS support

## Current Implementation

The current implementation includes:

1. **URL argument parsing** - Uses clap to handle command line arguments with extensive options
2. **HTML downloading** - Uses ureq HTTP client with rustls TLS support
3. **HTML table parsing** - Robust HTML table parsing without external HTML parsing dependencies
4. **CSV conversion** - Converts extracted table data to CSV format with customizable options:
   - Configurable delimiter (comma, pipe, semicolon, etc.)
   - Flexible quoting modes (never, always, as-needed)
   - Header removal option
   - Column-specific quoting
   - Column filtering to show only specified fields
5. **Error handling** - Proper error handling and user feedback

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

</details>

## Command Line Options

- `-d, --delimiter <CHAR>`: Change delimiter character (default: ',')
- `-q, --quote-fields <MODE>`: Quote fields mode: never, always, asneeded (default: asneeded)
- `--no-header`: Remove table headers from output
- `--quote-columns <COLUMNS>`: Only quote specific columns (comma-separated indices, e.g., '1,3')
- `--show-fields <COLUMNS>`: Show only specified columns (comma-separated indices, e.g., '1,3,5')

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
Name,Age,City
John Doe,25,New York
Jane Smith,30,Los Angeles
```

With different options:
```bash
# Using pipe delimiter
Name|Age|City
John Doe|25|New York
Jane Smith|30|Los Angeles

# With always quote
"Name","Age","City"
"John Doe","25","New York"
"Jane Smith","30","Los Angeles"

# Without header
John Doe,25,New York
Jane Smith,30,Los Angeles

# Show only columns 1 and 3 (Name and City)
Name,City
John Doe,New York
Jane Smith,Los Angeles
```
