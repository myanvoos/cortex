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
        
begin(python)
A = "hello world"
a = 5
b = 6
B = [ [1, a, b], [4, 5, 6], [[1, 2], 100, 1000] ]

def add(a, b):
    return a + b

def print_hello():
    return "hello world from inside a function"
end(python)

end(setup)
        
begin(document)
This is some text with math: >(A). This is another chunk of text just to test the formatting.

This is a matrix: $$(matrix B)

Printing a function: >(print_hello()). Hi!

Result of adding >(a) and >(b): >(add(5, 6))

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
