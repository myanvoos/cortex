// Using tokeniser to get tokens. Parser turns tokens into meaningful instructions

use crate::tokeniser;

pub fn parse(tokens: Vec<tokeniser::Token>) {
    let mut iterator = tokens.into_iter().peekable();

    while let Some(token) = iterator.next() {
        match token {
            tokeniser::Token::BeginSetup => {
                println!("\\begin{{setup}}");
            }
            tokeniser::Token::DocumentClass(class_name) => {
                println!("\\documentclass{{{}}}", class_name);
            }
            tokeniser::Token::Matrix => {
                println!("\\begin{{pmatrix}} a & b & c \\\\ d & e & f \\\\ g & h & i \\end{{pmatrix}}"); // placeholder
            }
            tokeniser::Token::Sum => {
                println!("\\sum_{{n=1}}^{{n-1}}"); // placeholder
            }
            tokeniser::Token::EndSetup => {
                println!("\\end{{setup}}");
            }
            _ => {
                println!("Unknown token: {:?}", token);
            }
        }
    }
}