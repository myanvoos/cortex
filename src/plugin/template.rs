use crate::parser::{LatexState, Rule};

pub fn basic_template(latex_state: &mut LatexState) -> Result<(), pest::error::Error<Rule>> {
    latex_state.append_to_body(
        format!("\\documentclass{{{}}}\n", latex_state.setup.document_class)
    );
    latex_state.append_to_body("\\usepackage{amsmath}\n".to_string());
    latex_state.append_to_body(
        format!("\\title{{{}}}\n", latex_state.document.title)
    );
    latex_state.append_to_body(
        format!("\\author{{{}}}\n", latex_state.document.author.join(", "))
    );
    latex_state.append_to_body("\\begin{document}\n".to_string());
    Ok(())
}