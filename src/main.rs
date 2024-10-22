// src/main.rs

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
        
        t = 1509

        asper = t

        A = [
            [a, b, c]
            [d, e, f]
            [g, h, i]
        ]

        B = [[1, 2, 3] [4, 5, 6]]
        
        end(setup)
        
        begin(document)
        This is some text with math: 

        $$(matrix A)
        $$(matrix B)
        # hi
        A variable is $(t). Text afterwards.
        
New line now: $$(asper)

$(fraction \dy\dx)

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
