use html_table_csv_converter::extract_table_from_html;

fn main() {
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
    
    match extract_table_from_html(test_html) {
        Ok(csv) => println!("CSV output:\n{}", csv),
        Err(e) => println!("Error: {}", e),
    }
}
