use html_table_csv_converter::{extract_table_from_html, extract_table_from_html_debug};

fn main() {
    let test_html = r#"
    <html>
    <body>
        <table>
            <tr>
                <th>IP</th>
                <th>Hostname</th>
                <th>In</th>
                <th>Out</th>
                <th>Total</th>
                <th>Last seen</th>
            </tr>
            <tr>
                <td>192.168.1.1</td>
                <td>router</td>
                <td>100</td>
                <td>200</td>
                <td>300</td>
                <td>2025-07-21</td>
            </tr>
            <tr>
                <td>192.168.1.100</td>
                <td>laptop</td>
                <td>50</td>
                <td>75</td>
                <td>125</td>
                <td>2025-07-21</td>
            </tr>
        </table>
    </body>
    </html>
    "#;
    
    println!("Testing with debug version:");
    match extract_table_from_html_debug(test_html) {
        Ok(csv) => println!("SUCCESS:\n{}", csv),
        Err(e) => println!("ERROR: {}", e),
    }
    
    println!("\nTesting with regular version:");
    match extract_table_from_html(test_html) {
        Ok(csv) => println!("SUCCESS:\n{}", csv),
        Err(e) => println!("ERROR: {}", e),
    }
}
