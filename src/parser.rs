use std::{collections::HashMap, error::Error, hash::Hash};

use pest::Parser;
use pest_derive::Parser;

use crate::plugin::build_preamble;

////////////////////////////////
/// STRUCTURE
///  
#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct LatexParser;

// SETUP: Defining a struct to hold all the state variables. These will be defined in setup block.
#[derive(Default, Debug)]
pub struct LatexState {
    pub setup: SetupBlock,
    pub document: DocumentBlock,
    pub matrices: HashMap<String, Matrix>,
    pub functions: HashMap<String, Function>,
    pub variables: HashMap<String, String>
}

#[derive(Debug)]
pub struct Function {
    pub params: Vec<String>,
    pub body: String
}


// TODO: Separate matrices for display and for doing maths with
// same for functions
#[derive(Debug)]
pub struct Matrix {
    pub rows: Vec<Vec<String>>
}

impl FromIterator<Vec<String>> for Matrix {
    fn from_iter<I: IntoIterator<Item = Vec<String>>>(iter: I) -> Self {
        Self {
            rows: iter.into_iter().collect()
        }
    }
}

#[derive(Default, Debug)]
pub struct SetupBlock {
    pub document_class: String,
    pub document_options: Vec<String>
}

#[derive(Default, Debug)]
pub struct DocumentBlock {
    // Might move this to setup to fit grammar
    // and make vectors with inline-math, newline-math, paragraphs etc
    // so that we can use them in document block and move them around
    pub title: String,
    pub author: Vec<String>,
    pub body: String
}

// SETUP: Implementing LatexState
impl LatexState {
    pub fn new() -> Self {
        Self::default()
    }
    // Helper methods to modify state of the latex
    pub fn set_title(&mut self, title: String) {
        self.document.title = title;
    }
    pub fn add_author(&mut self, author: String) {
        self.document.author.push(author);
    }
    pub fn set_document_class(&mut self, document_class: String) {
        self.setup.document_class = document_class;
    }
    pub fn set_document_options(&mut self, document_options: Vec<String>) {
        self.setup.document_options = document_options;
    }
    pub fn append_to_body(&mut self, body: String) {
        self.document.body.push_str(&body);
    }
    pub fn add_matrix_to_map(&mut self, name: String, matrix: Matrix) {
        self.matrices.insert(name, matrix);
    }
    pub fn add_function_to_map(&mut self, name: String, function: Function) {
        self.functions.insert(name, function);
    }
    pub fn add_variable_to_map(&mut self, name: String, value: String) {
        self.variables.insert(name, value);
    }
}

////////////////////////////////

pub fn parse_to_latex(input: &str) -> Result<String, pest::error::Error<Rule>> {
    let placeholder = "".to_string();

    // file = { SOI ~ setup_block ~ document_block ~ EOI }
    let pairs = LatexParser::parse(Rule::file, input)?;

    let mut state = LatexState::new();

    for pair in pairs {
        match pair.as_rule() {
            Rule::file => {
                // print_inner_pairs(pair);
                for inner_pair in pair.into_inner() {
                    match inner_pair.as_rule() {
                        Rule::setup_block => {
                            parse_setup_block(inner_pair, &mut state);
                        },
                        Rule::document_block => {
                            let mut latex = String::new();
                            let _ = build_preamble(&mut state);

                            // Print what we have so far for debugging
                            println!("\n{}", state.document.body);
                            
                            parse_document_block(inner_pair, &mut state);
                            return Ok(latex);
                        },
                        _ => {
                            println!("Unknown rule: {:?}", inner_pair.as_rule());
                        }
                    }
                }
            }
            _ => {
                println!("Unknown rule: {:?}", pair.as_rule());
            }
        }
    }


    Ok(placeholder)
}


fn parse_document_block(inner_pair: pest::iterators::Pair<Rule>, state: &mut LatexState) {
    for setup_pair in inner_pair.into_inner() {
        match setup_pair.as_rule() {
            Rule::document_class => {
                if let Some(content) = extracted_string_content(setup_pair.as_str()) {
                    println!("Extracted document class: {}", content);
                    state.set_document_class(content);
                }
            },
            Rule::author => {
                // ONLY WORKS FOR SINGLE AUTHORS FOR NOW
                if let Some(author) = extracted_string_content(setup_pair.as_str()) {
                    println!("Extracted author: {}", author);
                    state.add_author(author);
                }
            },
            Rule::title => {
                if let Some(title) = extracted_string_content(setup_pair.as_str()) {
                    println!("Extracted title: {}", title);
                    state.set_title(title);
                }
            },
            Rule::inline_math_expr | Rule::newline_math_expr=> {
                println!("Extracted math: {}", setup_pair.as_str()); 
                
            }
            _ => {}
        }
    }
}

fn parse_setup_block(inner_pair: pest::iterators::Pair<Rule>, state: &mut LatexState) -> Result<(), Box<dyn Error>>{
    for setup_pair in inner_pair.into_inner() {
        match setup_pair.as_rule() {
            Rule::matrix => {
                // state.append_to_body("\\begin{equation}\n".to_string());
                
                // TODO: Add support for other matrix types
                // Pmatrix for now
                // state.append_to_body("\\begin{pmatrix}\n".to_string());
                
                if let Some((name, matrix)) = extracted_matrix_content(setup_pair.as_str()) {
                    state.add_matrix_to_map(name, matrix);
                }
            },
            
            _ => {}
        }
    }
    Ok(())
}

pub fn process_maths(inner_pair: pest::iterators::Pair<Rule>, state: &mut LatexState) -> () {
    for process_pair in inner_pair.into_inner() {
        match process_pair.as_rule() {
            Rule::matrix => {
                state.append_to_body("\\begin{equation}\n".to_string());
                
                // TODO: Add support for other matrix types
                // Pmatrix for now
                state.append_to_body("\\begin{pmatrix}\n".to_string());
        
                
        
                state.append_to_body("\\end{pmatrix}\n".to_string());
                state.append_to_body("\\end{equation}\n".to_string())
            },
            _ => {}
        }
    }
}

// Helper function to parse matrices. We want a key-value pair
pub fn extracted_matrix_content(input: &str) -> Option<(String, Matrix)> {
    // Separate name and matrix content
    let parts: Vec<&str> = input.split("=").collect();
    if parts.len() != 2 {
        return None;
    }

    let name = parts[0].trim();

    let matrix_str = parts[1].trim();
    if !matrix_str.starts_with('[') || !matrix_str.ends_with(']') {
        return None;
    }

    // Remove outer brackets, split into rows
    let inner_content = &matrix_str[1..matrix_str.len() - 1].trim();
    if !inner_content.starts_with('[') || !inner_content.ends_with(']') {
        return None;
    }

    let matrix: Matrix = inner_content
        .split("]\n")
        .map(|row| {
            row.trim()
                .trim_matches(|c| c == '[' || c == ']')
                .split(',')
                .map(|element| {
                    element
                        .trim()
                        .trim_matches(|c| c == '[' || c == ']')
                        .to_string()
                })  
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>()
        })
        .filter(|row: &Vec<String>| !row.is_empty())
        .collect();

    if matrix.rows.is_empty() {
        return None;
    } else {
        Some((name.to_string(), matrix))
    }

}

// Helper function to remove quotes from a string
pub fn remove_quotes(s: &str) -> String {
    s.trim_matches(|c| c == '\"' || c == '\'').to_string()
}


// Helper function to extract string from pair
pub fn extracted_string_content(input: &str) -> Option<String> {
    let first_quote = input.find(|c| c == '"' || c == '\'');

    if let Some(start_index) = first_quote {
        let quote_char = input.chars().nth(start_index).unwrap();
        if let Some(end_index) = input[start_index + 1..].find(|c| c == quote_char) {
            let content = &input[start_index + 1..start_index + 1 + end_index];
            return Some(content.to_string());
        }
    }
    None
}

// DEBUG: Helper function to get the next inner pairs
fn print_inner_pairs(pair: pest::iterators::Pair<Rule>) {
    for inner_pair in pair.into_inner() {
        println!("{:?}", inner_pair.as_rule())
    }
}

// DEBUG: Helper function to recursively traverse through pair tree
fn print_all_pairs(pair: pest::iterators::Pair<Rule>, level: usize) {
    
    // Pair.as_rule() shows the current rule. Pair.as_span() shows the span or 'range' of the rule.
    // Reference: https://pest.rs/book/parser_api.html#pairs
    // Example (Parse tree) - 
    //       Rule::setup_block will span from `begin(setup)` to `end(setup)`
    //       Its 'parent' is Rule::file which spans the entire document
    //       Rule::document_class is a 'child' of Rule::setup_block

    println!("Rule: {:?}, Span: {:?}\n", pair.as_rule(), pair.as_span());

    // Gets the inner Pairs between the Pair
    let inner = pair.into_inner(); 

    for inner_pair in inner {
        print_all_pairs(inner_pair, level + 1);
    }
}