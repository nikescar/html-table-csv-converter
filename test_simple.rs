fn main() {
    println!("Testing basic functionality");
    
    // Test if we can create a simple CSV
    let test_html = r#"
        <html>
        <body>
            <table>
                <tr><th>Name</th><th>Age</th></tr>
                <tr><td>John</td><td>25</td></tr>
                <tr><td>Jane</td><td>30</td></tr>
            </table>
        </body>
        </html>
    "#;
    
    match html_table_csv_converter::extract_tables_to_csv(test_html) {
        Ok(csv) => println!("CSV:\n{}", csv),
        Err(e) => println!("Error: {}", e),
    }
}
