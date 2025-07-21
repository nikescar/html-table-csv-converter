// A robust HTML table parser without external dependencies
pub fn extract_table_from_html(html: &str) -> Result<String, String> {
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
            
            // Convert rows to CSV
            for row in rows {
                if !row.is_empty() {
                    csv_output.push_str(&row.join(","));
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
                
                if content_start < content_end {
                    let cell_content = &row_html[content_start..content_end];
                    let cleaned = clean_cell_content(cell_content);
                    cells.push(format!("\"{}\"", cleaned));
                }
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
    
    // Clean up whitespace and escape commas
    cleaned.trim()
        .replace('\n', " ")
        .replace('\r', " ")
        .replace('\t', " ")
        .replace(',', ";")
        .chars()
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ")
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
        assert!(result.contains("\"Name\",\"Age\",\"City\""));
        // Check that we have the data rows
        assert!(result.contains("\"John Doe\",\"25\",\"New York\""));
        assert!(result.contains("\"Jane Smith\",\"30\",\"Los Angeles\""));
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
        
        assert!(result.contains("\"IP\",\"Hostname\""));
        assert!(result.contains("\"192.168.1.1\",\"router\""));
    }
}
