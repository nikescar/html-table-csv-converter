// CSV configuration options
#[derive(Debug, Clone)]
pub struct CsvOptions {
    pub delimiter: char,
    pub quote_mode: QuoteMode,
    pub no_header: bool,
    pub quote_columns: Vec<usize>, // 1-based column indices
    pub show_fields: Vec<usize>,   // 1-based column indices to show
}

#[derive(Debug, Clone)]
pub enum QuoteMode {
    Never,
    Always,
    AsNeeded,
}

impl Default for CsvOptions {
    fn default() -> Self {
        Self {
            delimiter: ',',
            quote_mode: QuoteMode::AsNeeded,
            no_header: false,
            quote_columns: Vec::new(),
            show_fields: Vec::new(),
        }
    }
}

// A robust HTML table parser without external dependencies
pub fn extract_table_from_html(html: &str) -> Result<String, String> {
    extract_table_from_html_with_options(html, CsvOptions::default())
}

pub fn extract_table_from_html_with_options(html: &str, options: CsvOptions) -> Result<String, String> {
    let mut csv_output = String::new();
    
    // Convert to lowercase for case-insensitive matching
    let html_lower = html.to_lowercase();
    
    // Find all table elements
    let mut pos = 0;
    while let Some(table_start) = html_lower[pos..].find("<table") {
        let table_start = pos + table_start;
        
        // Find the end of the table
        if let Some(table_end) = html_lower[table_start..].find("</table>") {
            let table_end = table_start + table_end + 8; // +8 for "</table>"
            let table_html = &html[table_start..table_end];
            
            // Extract rows from this table
            let rows = extract_table_rows(table_html);
            
            // Skip header if no_header option is set
            let rows_to_process = if options.no_header && !rows.is_empty() {
                &rows[1..]
            } else {
                &rows
            };
            
            // Convert rows to CSV with options
            for row in rows_to_process {
                if !row.is_empty() {
                    let formatted_row = format_row_with_options(row, &options);
                    csv_output.push_str(&formatted_row);
                    csv_output.push('\n');
                }
            }
            
            pos = table_end;
        } else {
            break;
        }
    }
    
    if csv_output.is_empty() {
        Ok("No tables found in the HTML".to_string())
    } else {
        Ok(csv_output)
    }
}

fn extract_table_rows(table_html: &str) -> Vec<Vec<String>> {
    let mut rows = Vec::new();
    let table_lower = table_html.to_lowercase();
    
    let mut pos = 0;
    while let Some(row_start) = table_lower[pos..].find("<tr") {
        let row_start = pos + row_start;
        
        // Find the end of this row
        if let Some(row_end_tag) = table_lower[row_start..].find("</tr>") {
            let row_end = row_start + row_end_tag + 5; // +5 for "</tr>"
            let row_html = &table_html[row_start..row_end];
            
            // Extract cells from this row
            let cells = extract_row_cells(row_html);
            if !cells.is_empty() {
                rows.push(cells);
            }
            
            pos = row_end;
        } else {
            break;
        }
    }
    
    rows
}

fn extract_row_cells(row_html: &str) -> Vec<String> {
    let mut cells = Vec::new();
    let row_lower = row_html.to_lowercase();
    
    let mut pos = 0;
    loop {
        // Look for either <td or <th
        let td_pos = row_lower[pos..].find("<td");
        let th_pos = row_lower[pos..].find("<th");
        
        let cell_start = match (td_pos, th_pos) {
            (Some(td), Some(th)) => pos + td.min(th),
            (Some(td), None) => pos + td,
            (None, Some(th)) => pos + th,
            (None, None) => break,
        };
        
        // Find the closing tag
        let is_th = row_lower[cell_start..].starts_with("<th");
        let close_tag = if is_th { "</th>" } else { "</td>" };
        
        if let Some(cell_end_tag) = row_lower[cell_start..].find(close_tag) {
            let cell_end = cell_start + cell_end_tag + close_tag.len();
            
            // Find the actual content start (after the opening tag)
            if let Some(content_start) = row_lower[cell_start..].find('>') {
                let content_start = cell_start + content_start + 1;
                let content_end = cell_start + cell_end_tag;
                
                let cell_content = if content_start < content_end {
                    &row_html[content_start..content_end]
                } else {
                    "" // Empty cell
                };
                let cleaned = clean_cell_content(cell_content);
                cells.push(cleaned);
            } else {
                // Malformed cell, but still add empty string to maintain column count
                cells.push(String::new());
            }
            
            pos = cell_end;
        } else {
            break;
        }
    }
    
    cells
}

fn clean_cell_content(content: &str) -> String {
    // Remove any HTML tags that might be inside the cell
    let mut cleaned = String::new();
    let mut in_tag = false;
    
    for ch in content.chars() {
        if ch == '<' {
            in_tag = true;
        } else if ch == '>' {
            in_tag = false;
        } else if !in_tag {
            cleaned.push(ch);
        }
    }
    
    // Clean up whitespace 
    cleaned.trim()
        .replace('\n', " ")
        .replace('\r', " ")
        .replace('\t', " ")
        .chars()
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ")
}

fn format_row_with_options(row: &[String], options: &CsvOptions) -> String {
    // Filter columns if show_fields is specified
    let row_to_process: Vec<String> = if !options.show_fields.is_empty() {
        options.show_fields
            .iter()
            .map(|&col_index| {
                if col_index > 0 && col_index <= row.len() {
                    row[col_index - 1].clone() // Convert 1-based to 0-based indexing
                } else {
                    String::new() // Return empty string for missing columns
                }
            })
            .collect()
    } else {
        row.to_vec()
    };

    let formatted_cells: Vec<String> = row_to_process
        .iter()
        .enumerate()
        .map(|(displayed_index, cell)| {
            // For quoting logic, we need the original column index
            let original_index = if !options.show_fields.is_empty() {
                // Find the original index for this displayed column
                options.show_fields.get(displayed_index).copied().unwrap_or(1) - 1
            } else {
                displayed_index
            };

            let should_quote = match options.quote_mode {
                QuoteMode::Never => false,
                QuoteMode::Always => true,
                QuoteMode::AsNeeded => {
                    // Quote if contains delimiter, quotes, or newlines
                    cell.contains(options.delimiter) || cell.contains('"') || cell.contains('\n')
                }
            };
            
            // Override with specific column quoting if specified
            let should_quote = if !options.quote_columns.is_empty() {
                options.quote_columns.contains(&(original_index + 1)) // 1-based indexing
            } else {
                should_quote
            };
            
            if should_quote {
                format!("\"{}\"", cell.replace('"', "\"\"")) // Escape quotes
            } else {
                cell.clone()
            }
        })
        .collect();
    
    formatted_cells.join(&options.delimiter.to_string())
}

// Download HTML from a URL using ureq
pub fn download_html(url: &str) -> Result<String, String> {
    match ureq::get(url).call() {
        Ok(mut response) => {
            let status = response.status();
            if status == 200 {
                match response.body_mut().read_to_string() {
                    Ok(content) => Ok(content),
                    Err(e) => Err(format!("Failed to read response body: {}", e)),
                }
            } else {
                Err(format!("HTTP request failed with status: {}", status))
            }
        }
        Err(e) => Err(format!("Failed to make HTTP request to {}: {}", url, e)),
    }
}

pub fn process_url_to_csv(url: &str) -> Result<String, String> {
    let html = download_html(url)?;
    let csv = extract_table_from_html(&html)?;
    Ok(csv)
}

pub fn process_url_to_csv_with_options(url: &str, options: CsvOptions) -> Result<String, String> {
    let html = download_html(url)?;
    let csv = extract_table_from_html_with_options(&html, options)?;
    Ok(csv)
}

// Debug version of the HTML parser to help troubleshoot
pub fn extract_table_from_html_debug(html: &str) -> Result<String, String> {
    let mut csv_output = String::new();
    let mut in_table = false;
    let mut in_row = false;
    let mut in_cell = false;
    let mut cell_content = String::new();
    let mut row_data = Vec::new();
    
    let mut chars = html.chars().peekable();
    let mut tag_buffer = String::new();
    let mut in_tag = false;
    
    eprintln!("DEBUG: Starting HTML parsing...");
    eprintln!("DEBUG: HTML length: {} characters", html.len());
    
    while let Some(ch) = chars.next() {
        if ch == '<' {
            in_tag = true;
            tag_buffer.clear();
            continue;
        }
        
        if ch == '>' && in_tag {
            in_tag = false;
            let tag = tag_buffer.to_lowercase();
            
            // Extract just the tag name, ignoring attributes
            let tag_name = tag.split_whitespace().next().unwrap_or("");
            
            if tag_name == "table" {
                in_table = true;
                eprintln!("DEBUG: Found table start");
            } else if tag_name == "/table" && in_table {
                in_table = false;
                eprintln!("DEBUG: Found table end");
            } else if tag_name == "tr" && in_table {
                in_row = true;
                row_data.clear();
                eprintln!("DEBUG: Found row start");
            } else if tag_name == "/tr" && in_row {
                in_row = false;
                if !row_data.is_empty() {
                    let row_csv = row_data.join(",");
                    eprintln!("DEBUG: Adding row: {}", row_csv);
                    csv_output.push_str(&row_csv);
                    csv_output.push('\n');
                }
            } else if (tag_name == "td" || tag_name == "th") && in_row {
                in_cell = true;
                cell_content.clear();
                eprintln!("DEBUG: Found cell start ({})", tag_name);
            } else if (tag_name == "/td" || tag_name == "/th") && in_cell {
                in_cell = false;
                // Clean up the cell content and add to row
                let cleaned = cell_content.trim().replace(',', ";").replace('\n', " ");
                let quoted = format!("\"{}\"", cleaned);
                eprintln!("DEBUG: Adding cell: {}", quoted);
                row_data.push(quoted);
            }
            continue;
        }
        
        if in_tag {
            tag_buffer.push(ch);
        } else if in_cell {
            if !ch.is_control() || ch == ' ' {
                cell_content.push(ch);
            }
        }
    }
    
    eprintln!("DEBUG: Final CSV output length: {} characters", csv_output.len());
    
    if csv_output.is_empty() {
        Ok("No tables found in the HTML".to_string())
    } else {
        Ok(csv_output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_table_from_html() {
        let test_html = r#"
        <html>
        <body>
            <table class="some-class">
                <tr>
                    <th>Name</th>
                    <th>Age</th>
                    <th>City</th>
                </tr>
                <tr>
                    <td>John Doe</td>
                    <td>25</td>
                    <td>New York</td>
                </tr>
                <tr>
                    <td>Jane Smith</td>
                    <td>30</td>
                    <td>Los Angeles</td>
                </tr>
            </table>
        </body>
        </html>
        "#;
        
        let result = extract_table_from_html(test_html).unwrap();
        println!("Test result:\n{}", result);
        
        // Check that we have the header row
        assert!(result.contains("Name,Age,City"));
        // Check that we have the data rows
        assert!(result.contains("John Doe,25,New York"));
        assert!(result.contains("Jane Smith,30,Los Angeles"));
    }

    #[test]
    fn test_simple_table() {
        let simple_html = r#"<html><body>
<table>
<tr><th>IP</th><th>Hostname</th></tr>
<tr><td>192.168.1.1</td><td>router</td></tr>
</table>
</body></html>"#;
        
        let result = extract_table_from_html(simple_html).unwrap();
        println!("Simple test result:\n{}", result);
        
        assert!(result.contains("IP,Hostname"));
        assert!(result.contains("192.168.1.1,router"));
    }

    #[test]
    fn test_csv_options() {
        let test_html = r#"
        <table>
            <tr><th>Name</th><th>Description</th></tr>
            <tr><td>Test Item</td><td>Item with, comma</td></tr>
            <tr><td>Test|Pipe</td><td>Normal text</td></tr>
        </table>
        "#;
        
        // Test with always quote
        let options = CsvOptions {
            delimiter: ',',
            quote_mode: QuoteMode::Always,
            no_header: false,
            quote_columns: Vec::new(),
            show_fields: Vec::new(),
        };
        let result = extract_table_from_html_with_options(test_html, options).unwrap();
        assert!(result.contains("\"Name\",\"Description\""));
        assert!(result.contains("\"Test Item\",\"Item with, comma\""));
        
        // Test with pipe delimiter  
        let options = CsvOptions {
            delimiter: '|',
            quote_mode: QuoteMode::AsNeeded,
            no_header: false,
            quote_columns: Vec::new(),
            show_fields: Vec::new(),
        };
        let result = extract_table_from_html_with_options(test_html, options).unwrap();
        println!("Pipe delimiter result:\n{}", result);
        assert!(result.contains("Name|Description"));
        // When using pipe delimiter, comma doesn't need quoting but pipe does
        assert!(result.contains("Test Item|Item with, comma"));
        assert!(result.contains("\"Test|Pipe\"|Normal text"));
        
        // Test no header
        let options = CsvOptions {
            delimiter: ',',
            quote_mode: QuoteMode::AsNeeded,
            no_header: true,
            quote_columns: Vec::new(),
            show_fields: Vec::new(),
        };
        let result = extract_table_from_html_with_options(test_html, options).unwrap();
        assert!(!result.contains("Name,Description"));
        assert!(result.contains("Test Item,\"Item with, comma\""));
        
        // Test show fields - only column 1 (Name)
        let options = CsvOptions {
            delimiter: ',',
            quote_mode: QuoteMode::AsNeeded,
            no_header: false,
            quote_columns: Vec::new(),
            show_fields: vec![1],
        };
        let result = extract_table_from_html_with_options(test_html, options).unwrap();
        println!("Show fields result:\n{}", result);
        assert!(result.contains("Name"));
        assert!(result.contains("Test Item"));
        assert!(result.contains("Test|Pipe")); // No quotes needed since we're using comma delimiter
        // Should not contain the Description column (column 2)
        assert!(!result.contains("Description"));
        assert!(!result.contains("Item with, comma"));
    }
}
