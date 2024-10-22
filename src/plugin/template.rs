use crate::parser::Rule;

pub fn basic_template(latex: &mut String) -> Result<(), pest::error::Error<Rule>> {
    latex.push_str("\\documentclass{article}\n");
    latex.push_str("\\usepackage{amsmath}\n");
    latex.push_str("\\begin{document}\n");
    latex.push_str("\\title{Title}\n");
    latex.push_str("\\author{Author}\n");
    latex.push_str("\\begin{equation}\n");
    
    Ok(())
}