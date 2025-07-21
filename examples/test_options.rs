use html_table_csv_converter::{extract_table_from_html_with_options, CsvOptions, QuoteMode};

fn main() {
    let test_html = r#"
    <table>
        <tr><th>Product</th><th>Price</th><th>Notes</th></tr>
        <tr><td>Widget A</td><td>$19.99</td><td>Has, comma</td></tr>
        <tr><td>Gadget|B</td><td>$29.99</td><td>Has pipe</td></tr>
    </table>
    "#;

    println!("=== Default options ===");
    let result = html_table_csv_converter::extract_table_from_html(test_html).unwrap();
    println!("{}", result);

    println!("\n=== Pipe delimiter ===");
    let options = CsvOptions {
        delimiter: '|',
        quote_mode: QuoteMode::AsNeeded,
        no_header: false,
        quote_columns: Vec::new(),
        show_fields: Vec::new(),
    };
    let result = extract_table_from_html_with_options(test_html, options).unwrap();
    println!("{}", result);

    println!("\n=== Always quote ===");
    let options = CsvOptions {
        delimiter: ',',
        quote_mode: QuoteMode::Always,
        no_header: false,
        quote_columns: Vec::new(),
        show_fields: Vec::new(),
    };
    let result = extract_table_from_html_with_options(test_html, options).unwrap();
    println!("{}", result);

    println!("\n=== No header ===");
    let options = CsvOptions {
        delimiter: ',',
        quote_mode: QuoteMode::AsNeeded,
        no_header: true,
        quote_columns: Vec::new(),
        show_fields: Vec::new(),
    };
    let result = extract_table_from_html_with_options(test_html, options).unwrap();
    println!("{}", result);

    println!("\n=== Quote only column 1 ===");
    let options = CsvOptions {
        delimiter: ',',
        quote_mode: QuoteMode::AsNeeded,
        no_header: false,
        quote_columns: vec![1],
        show_fields: Vec::new(),
    };
    let result = extract_table_from_html_with_options(test_html, options).unwrap();
    println!("{}", result);

    println!("\n=== Show only columns 1 and 3 ===");
    let options = CsvOptions {
        delimiter: ',',
        quote_mode: QuoteMode::AsNeeded,
        no_header: false,
        quote_columns: Vec::new(),
        show_fields: vec![1, 3],
    };
    let result = extract_table_from_html_with_options(test_html, options).unwrap();
    println!("{}", result);
}
