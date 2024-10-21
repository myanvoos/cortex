// Define the token enum

#[derive(Debug, PartialEq)]
pub enum Token {
    BeginSetup, 
    // EndSetup, 
    DocumentClass(String),
    Matrix,
    Sum,
    // Fraction,
    Number(f64),
    Identifier(String),
    // Operator(String),
    ParenthesisOpen,
    ParenthesisClose,
    EndOfFile
}

// Takes the raw input string and converts it into tokens
pub fn tokenise(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable(); // Converts to character iterator that can be peeked at - allowing us to look at the next character without consuming it

    while let Some(&char) = chars.peek() {
        // Pattern matching using match to check each character and decide what token it corresponds to
        match char {
            '(' => {
                tokens.push(Token::ParenthesisOpen);
                chars.next();
            },
            ')' => {
                tokens.push(Token::ParenthesisClose);
                chars.next();
            },
            '0'..='9' => {
                let mut number = String::new();
                while let Some(&digit) = chars.peek() {
                    if digit.is_numeric() || digit == '.' {
                        number.push(digit);
                        chars.next();
                    } else {
                        break;
                    }
                }
                let value: f64 = number.parse().unwrap();
                tokens.push(Token::Number(value));
            }
            'a'..='z' | 'A'..='Z' => {
                let mut identifier = String::new();
                while let Some(&letter) = chars.peek() {
                    if letter.is_alphanumeric() {
                        identifier.push(letter);
                        chars.next();
                    } else {
                        break;
                    }
                }
                match identifier.as_str() {
                    // Match identifiers to legal ones
                    "begin" => {
                        // Block beginning keywords
                        let mut block_name = String::new();
                        chars.next(); // Skip '('
                        while let Some(&block_char) = chars.peek() {
                            if block_char == ')' {
                                chars.next();
                                break;
                            } else {
                                block_name.push(block_char);
                                chars.next();
                            }
                        }
                        match block_name.as_str() {
                            "setup" => tokens.push(Token::BeginSetup),
                            _ => println!("Unknown block: {}", block_name),
                        }
                    }
                    "documentclass" => tokens.push(Token::DocumentClass("article".to_string())), // hard=coded for now
                    "matrix" => tokens.push(Token::Matrix),
                    "sum" => tokens.push(Token::Sum),
                    _ => tokens.push(Token::Identifier(identifier)),
                }
            }
            _ => {
                // umknown characters
                chars.next();
            }
        }
    }
    tokens.push(Token::EndOfFile);
    tokens
}