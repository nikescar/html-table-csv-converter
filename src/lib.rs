use std::io::{self, Write};

// A simple HTML table parser without external dependencies
pub fn extract_table_from_html(html: &str) -> Result<String, String> {
    let mut csv_output = String::new();
    let mut in_table = false;
    let mut in_row = false;
    let mut in_cell = false;
    let mut cell_content = String::new();
    let mut row_data = Vec::new();
    
    let mut chars = html.chars().peekable();
    let mut tag_buffer = String::new();
    let mut in_tag = false;
    
    while let Some(ch) = chars.next() {
        if ch == '<' {
            in_tag = true;
            tag_buffer.clear();
            continue;
        }
        
        if ch == '>' && in_tag {
            in_tag = false;
            let tag = tag_buffer.to_lowercase();
            
            if tag == "table" {
                in_table = true;
            } else if tag == "/table" && in_table {
                in_table = false;
                break;
            } else if tag == "tr" && in_table {
                in_row = true;
                row_data.clear();
            } else if tag == "/tr" && in_row {
                in_row = false;
                if !row_data.is_empty() {
                    csv_output.push_str(&row_data.join(","));
                    csv_output.push('\n');
                }
            } else if (tag == "td" || tag == "th") && in_row {
                in_cell = true;
                cell_content.clear();
            } else if (tag == "/td" || tag == "/th") && in_cell {
                in_cell = false;
                // Clean up the cell content and add to row
                let cleaned = cell_content.trim().replace(',', ";").replace('\n', " ");
                row_data.push(format!("\"{}\"", cleaned));
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
    
    if csv_output.is_empty() {
        Ok("No tables found in the HTML".to_string())
    } else {
        Ok(csv_output)
    }
}

// Simulate downloading HTML (in a real implementation, you'd use reqwest)
pub fn download_html(url: &str) -> Result<String, String> {
    // For demonstration, return a sample HTML with a table
    let sample_html = format!(r#"
    <html>
    <head><title>Sample Page for {}</title></head>
    <body>
        <h1>Sample Data</h1>
        <table>
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
            <tr>
                <td>Bob Johnson</td>
                <td>35</td>
                <td>Chicago</td>
            </tr>
        </table>
    </body>
    </html>
    "#, url);
    
    Ok(sample_html)
}

pub fn process_url_to_csv(url: &str) -> Result<String, String> {
    let html = download_html(url)?;
    let csv = extract_table_from_html(&html)?;
    Ok(csv)
}
