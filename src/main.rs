// src/main.rs

mod parser;
mod plugin;
mod tests;

use pyo3::prelude::*;

use crate::parser::parse_to_latex;

// use demoparser::parse_to_latex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = r#"
        begin(setup)
        documentclass("article")
        title("One of the first men on earth")
        author("Emrys")
        
// Python chunk
A = "hello world"

a = int(6)
b = int(5)

def add(a, b):
    return a + b

def print_hello():
    return "hello world from inside a function"
// End of Python chunk

        end(setup)
        
        begin(document)
        This is some text with math: $(A)

        $(print_hello())
        Result from operation: $(add(100, 5))

        Also adding $(a) + $(b)

$(fraction \dy\dx)

        end(document)
    "#;

    pyo3::prepare_freethreaded_python();

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
