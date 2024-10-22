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

        B = [[1, 2, 3] [4, 5, 6]]

        C= [
            [a, 2, b] [4, c, 6]]

        D = [ [10, 100, 1000]
        [7,8,2]]
        
        end(setup)
        
        begin(document)
        This is some text with math: 

        $$(matrix A)
        $$(matrix B)
        $$(matrix C)
        $$(matrix D)

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
