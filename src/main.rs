// src/main.rs

mod demoparser;
mod parser;
mod plugin;
mod tests;

use crate::parser::parse_to_latex;

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

    match parse_to_latex(input) {
        Ok(latex) => {
            println!("{}", latex);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }

    Ok(())
}
