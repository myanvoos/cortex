use std::any::Any;
use std::{collections::HashMap, error::Error, hash::Hash};
use regex::Regex;
use pest::Parser;
use pest_derive::Parser;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};


use crate::plugin::build_preamble;

////////////////////////////////
/// STRUCTURE
///  
#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct LatexParser;

// SETUP: Defining a struct to hold all the state variables. These will be defined in setup block.
#[derive(Debug)]
pub struct LatexState {
    pub setup: SetupBlock,
    pub document: DocumentBlock,
    pub matrices: HashMap<String, Matrix>,
    pub functions: HashMap<String, Function>,
    pub variables: HashMap<String, String>,

    pub py_locals: Py<PyDict>
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

        // PYO3: Get a Python<'py> token using Python::with_gil
        // This allows us to access the Python interpreter.
        // Main objective here is to initialise the Python dictionary for storing local variables.

        // Reference: https://docs.rs/pyo3/0.22.5/pyo3/marker/struct.Python.html#obtaining-a-python-token

        Python::with_gil(|py| {
            Self {
                py_locals: PyDict::new_bound(py).into(),
                setup: SetupBlock::default(),
                document: DocumentBlock::default(),

                // Remove later because redundant
                matrices: HashMap::new(),
                functions: HashMap::new(),
                variables: HashMap::new(),
            }
        })
    }
    pub fn initialise_python_setup(&self, code: &str) -> Result<(), String> {

        // TODO: Add further string processing for distinguishing between types. 
        // Right now everything is treated as a string
        
        Python::with_gil(|py| {

            // NOTE FOR PERSONAL REFERENCE: First attaches the Python context py to self.py_locals
            // Then py.run_bound to execute one or more Python statements in code
            // It will return a PyDict (or similar Python struct) where each key:value corresponds to the variable name and value(s)

            let locals = self.py_locals.bind(py);
            py.run_bound(code, None, Some(locals)).unwrap();

            println!("Local py dict: {:?}", self.py_locals.to_string());
        });
        Ok(())
    }

    pub fn evaluate_python_code(&mut self, code: &str) {
        Python::with_gil(|py| {
            let locals = self.py_locals.bind(py);
                        
            // First try to evaluate as an expression, if fail then evaluate as a statement
            match py.eval_bound(code, None, Some(locals)) {
                Ok(result) => {
                    println!("Python result: {}", result);
                    self.append_to_body(result.to_string());
                },
                Err(_) => {
                    if let Err(e) = py.run_bound(code, None, Some(locals)) {
                        println!("Python execution error: {}", e);
                        self.append_to_body(format!("Error: {}", e));
                    }
                }
            }
        });
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
                            let _ = parse_setup_block(inner_pair, &mut state);
                        },
                        Rule::document_block => {

                            // DEBUG: newline at the start
                            state.append_to_body("\n".to_string());

                            let _ = build_preamble(&mut state);

                            // DEBUG: Print what we have so far for debugging
                            // println!("\n{}", state.document.body);
                            
                            parse_document_block(inner_pair, &mut state);

                            state.append_to_body("\\end{document}\n".to_string());
                            return Ok(state.document.body);
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
    for document_pair in inner_pair.into_inner() {
        match document_pair.as_rule() {
            Rule::text => {

                // TODO: Make this into its own function
                let text = document_pair.as_str().trim();
                let first_char = text.chars().next().unwrap();
                
                if first_char.is_ascii_uppercase() || first_char == '.' || first_char == ':' {
                    state.append_to_body(format!("{} ", text.to_string()));
                } else if first_char.is_ascii_lowercase() {
                    state.append_to_body(format!(" {} ", text.to_string()));
                } else {
                    state.append_to_body(format!("{}", text.to_string()));
                }
            }
            Rule::inline_math_expr | Rule::newline_math_expr=> {
                let delimiter = if document_pair.as_rule() == Rule::inline_math_expr {
                    "$"
                } else {
                    "$$"
                };

                if delimiter == "$$" {
                    state.append_to_body(format!("\n{}", delimiter.to_string()));
                } else {
                    state.append_to_body(format!(" {}", delimiter.to_string()));
                }
                
                process_maths(document_pair, state);
                
                if delimiter == "$$" {
                    state.append_to_body(format!("{}\n", delimiter.to_string()));
                } else {
                    state.append_to_body(format!("{}", delimiter.to_string()));
                }
            },
            Rule::code_output => {
                process_code(document_pair, state);
            }
            _ => {}
        }
    }
}

fn process_code(inner_pair: pest::iterators::Pair<Rule>, state: &mut LatexState) {
    for code_pair in inner_pair.into_inner() {
        match code_pair.as_rule() {
            Rule::allowed_python_code_in_document => {
                let code = code_pair.as_str();
                state.evaluate_python_code(code);
            }
            _ => {}
        }
    }
}

fn parse_setup_block(inner_pair: pest::iterators::Pair<Rule>, state: &mut LatexState) -> Result<(), Box<dyn Error>>{
    for setup_pair in inner_pair.into_inner() {
        match setup_pair.as_rule() {
            Rule::document_class => {
                if let Some(content) = extract_string_content(setup_pair.as_str()) {
                    state.set_document_class(content);
                }
            },
            Rule::author => {
                // ONLY WORKS FOR SINGLE AUTHORS FOR NOW
                if let Some(author) = extract_string_content(setup_pair.as_str()) {
                    state.add_author(author);
                }
            },
            Rule::title => {
                if let Some(title) = extract_string_content(setup_pair.as_str()) {
                    state.set_title(title);
                }
            },
            Rule::python_block => {
                
                // Right now only one inner rule. 
                // If we're adding import rule then need to change this for safety. 
                let python_code = setup_pair.into_inner().as_str();
                let _ = state.initialise_python_setup(python_code);
            }
            
            _ => {}
        }
    }
    Ok(())
}

pub fn process_maths(inner_pair: pest::iterators::Pair<Rule>, state: &mut LatexState) {
    // Nested function to recursively process expressions
    fn process_expression(pair: pest::iterators::Pair<Rule>, state: &mut LatexState) {
        match pair.as_rule() {
            Rule::matrix => {
                state.append_to_body("\\begin{equation}\n".to_string());
                state.append_to_body("\\begin{pmatrix}\n".to_string());

                let identifier = pair
                    .into_inner()
                    .find(|p| p.as_rule() == Rule::identifier)
                    .map(|p| p.as_str())
                    .unwrap_or("");

                if !identifier.is_empty() {
                    let _ = Python::with_gil(|py| {
                        match get_matrix_elements(identifier, state, py) {
                            Ok(elements) => {
                                let mut formatted_rows = Vec::new();
                                for row in elements {
                                    let row_str = row.join(" & ");
                                    formatted_rows.push(format!("{} \\\\\n", row_str));
                                }

                                state.append_to_body(formatted_rows.join(""));
                            },
                            Err(e) => {
                                println!("Error processing matrix: {:?}", e);
                            }
                        }
                        Ok::<(), ()>(())
                    });
                }

                state.append_to_body("\\end{pmatrix}\n".to_string());
                state.append_to_body("\\end{equation}\n".to_string());
            },
            // Handle other expression types by recursively processing their children
            Rule::sum_expression |
            Rule::product_expression |
            Rule::power_expression |
            Rule::fraction |
            Rule::primary_expression => {
                for child in pair.into_inner() {
                    process_expression(child, state);
                }
            },
            // If it's a terminal node (number, identifier)
            Rule::number |
            Rule::identifier => {
                let content = pair.as_str();
                state.append_to_body(content.to_string());
            },
            _ => {}
        }
    }

    // Start processing from the top level
    for pair in inner_pair.into_inner() {
        process_expression(pair, state);
    }
}


fn get_matrix_elements(identifier: &str, state: &LatexState, py: Python) -> PyResult<Vec<Vec<String>>> {
    let locals = state.py_locals.bind(py);
    let var = locals.get_item(identifier).unwrap().expect("Variable not found");

    if var.hasattr("shape")? {
        // It's a NumPy array
        let matrix = var;
        let shape = matrix.getattr("shape")?;
        let rows = shape.get_item(0)?.extract::<usize>()?;
        let cols = shape.get_item(1)?.extract::<usize>()?;

        let mut elements = Vec::new();
        for i in 0..rows {
            let mut row = Vec::new();
            for j in 0..cols {
                let element = matrix.get_item((i, j))?;
                let formatted_element = element.str()?.to_str()?.to_string();
                row.push(formatted_element);
            }
            elements.push(row);
        }
        Ok(elements)
    } else if var.is_instance_of::<PyList>() {
        // It's a list of lists
        let py_list = var.downcast::<PyList>()?;
        let mut elements = Vec::new();
        for item in py_list.iter() {
            if item.is_instance_of::<PyList>() {
                let row_list = item.downcast::<PyList>()?;
                let mut row = Vec::new();
                for element in row_list.iter() {
                    let formatted_element = element.str()?.to_str()?.to_string();
                    row.push(formatted_element);
                }
                elements.push(row);
            } else {
                // Single element, wrap it in a row
                let formatted_element = item.str()?.to_str()?.to_string();
                elements.push(vec![formatted_element]);
            }
        }
        Ok(elements)
    } else {
        // Not a matrix
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(format!("Variable {} is not a matrix", identifier)))
    }
}


// Helper function to extract string from pair
pub fn extract_string_content(input: &str) -> Option<String> {
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