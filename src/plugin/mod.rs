pub mod template;
// pub mod custom_template;

use crate::parser::Rule;

pub fn build_preamble(latex: &mut String) -> Result<(), pest::error::Error<Rule>> {
    template::basic_template(latex)
}