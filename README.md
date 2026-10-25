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

</details>

## Getting Started

```bash
# Install with cargo
$ cargo install --git https://github.com/nikescar/html-table-csv-converter.git

# Basic usage - download and convert tables from a URL
$ html-table-csv-converter "https://example.com/page-with-tables"

# Use pipe delimiter instead of comma
$ html-table-csv-converter -d "|" "https://example.com/page-with-tables"

# Always quote all fields
$ html-table-csv-converter --quote-fields always "https://example.com/page-with-tables"

# Remove headers from output
$ html-table-csv-converter --no-header "https://example.com/page-with-tables"

# Quote only specific columns (1-based indexing)
$ html-table-csv-converter --quote-columns "1,3" "https://example.com/page-with-tables"

# Show only specific columns
$ html-table-csv-converter --show-fields "1,3" "https://example.com/page-with-tables"

# Combine options
$ html-table-csv-converter -d ";" --quote-fields never --no-header --show-fields "2,4" "https://example.com/page-with-tables"

```

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
