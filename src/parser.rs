

use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct LatexParser;

pub fn parse_to_latex(input: &str) -> Result<String, pest::error::Error<Rule>> {
    let mut placeholder = "".to_string();
        
    Ok(placeholder)
}