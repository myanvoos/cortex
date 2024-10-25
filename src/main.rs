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
title("Title")
author("Author1", "Author2")
        
begin(python)
A = "hello world"
a = 5
b = 6
B = [ [1, a, b], [4, 5, 6], [[1, 2], 100, 1000] ]

def add(a, b):
    return a + b

def print_hello():
    return "hello world from inside a function"

class Person:
    def __init__(self, name, age):
        self.name = name
        self.age = age
    def __str__(self):
        return f"Name: {self.name}, Age: {self.age}"
    def get_name(self):
        return self.name
    def get_age(self):
        return self.age

p1 = Person("John", 36)

end(python)

end(setup)
        
begin(document)
This is some text with math: >(A). This is another chunk of text just to test the formatting.

This is a matrix: $$(matrix B)

Printing a function: >(print_hello()). Hi!

Person object: >(p1).

>(p1.get_name()) is a person!

You can add >(a) and >(b) to get >(a + b). Or you can call the function >(add(a, b))!

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
