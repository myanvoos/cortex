pub mod template;
// pub mod custom_template;

use crate::parser::{LatexState, Rule};

pub fn build_preamble(latex_state: &mut LatexState) -> Result<(), pest::error::Error<Rule>> {
    template::basic_template(latex_state)
}