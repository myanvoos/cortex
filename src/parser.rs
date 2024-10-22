

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct LatexParser;

pub fn parse_to_latex(input: &str) -> Result<String, pest::error::Error<Rule>> {
    let placeholder = "".to_string();

    // file = { SOI ~ setup_block ~ document_block ~ EOI }
    let pairs = LatexParser::parse(Rule::file, input)?;

    // print_all_pairs(pairs, 0);

    for pair in pairs {
        match pair.as_rule() {
            Rule::file => {
                // print_inner_pairs(pair);
                for inner_pair in pair.into_inner() {
                    match inner_pair.as_rule() {
                        Rule::setup_block => {
                            parse_setup_block(&inner_pair);
                        },
                        Rule::document_block => {
                            let mut latex = String::new();
                            build_preamble(&mut latex);
                            parse_document_block(&inner_pair, &mut latex);
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

fn build_preamble(latex: &mut String) {
    
}

fn parse_document_block(inner_pair: &pest::iterators::Pair<Rule>, latex: &mut String) {

}

fn parse_setup_block(inner_pair: &pest::iterators::Pair<Rule>) {

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