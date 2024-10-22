// src/main.rs

mod parser;

// use demoparser::parse_to_latex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = r#"
        begin(setup)
        documentclass("article")
        title("One of the first men on earth")
        author("Emrys")
        
        A = [
            [a, b, c]
            [d, e, f]
            [g, h, i]
        ]
        
        end(setup)
        
        begin(document)
        This is some text with math: $(matrix A)
        end(document)
    "#;

    Ok(())
}
