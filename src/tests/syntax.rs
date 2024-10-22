#[cfg(test)]
mod test {
    use crate::tokeniser::{self, Token};
    use super::*;

    fn debug_tokens(actual: &[Token], expected: &[Token]) {
        println!("\nTokens comparison:");
        println!("Length: actual={}, expected={}", actual.len(), expected.len());
        
        for (i, (a, e)) in actual.iter().zip(expected.iter()).enumerate() {
            if a != e {
                println!("Mismatch at position {}:", i);
                println!("  Actual: {:?}", a);
                println!("Expected: {:?}", e);
            }
        }

        // Print any remaining tokens if lengths don't match
        if actual.len() > expected.len() {
            println!("\nExtra tokens in actual:");
            for token in actual.iter().skip(expected.len()) {
                println!("  {:?}", token);
            }
        } else if expected.len() > actual.len() {
            println!("\nMissing tokens:");
            for token in expected.iter().skip(actual.len()) {
                println!("  {:?}", token);
            }
        }
    }

    #[test]
    fn test_token() {
        let input = "
        begin(setup)
        documentclass('article')
        A = [
            [a, b, c]
            [d, e, f]
        ]
        $(sum (n -> n-1) \\gx)
        end(setup)
        ";

        let tokens = tokeniser::tokenise(input);
        let expected_tokens = vec![
            Token::BeginSetup,
            Token::DocumentClass("article".to_string()),
            Token::ParenthesisOpen,
            Token::Identifier("article".to_string()),
            Token::ParenthesisClose,
            Token::Identifier("A".to_string()),
            Token::Identifier("a".to_string()),
            Token::Identifier("b".to_string()),
            Token::Identifier("c".to_string()),
            Token::Identifier("d".to_string()),
            Token::Identifier("e".to_string()),
            Token::Identifier("f".to_string()),
            Token::ParenthesisOpen,
            Token::Sum,
            Token::ParenthesisOpen,
            Token::Identifier("n".to_string()),
            Token::Identifier("n".to_string()),
            Token::Number(1.0),
            Token::ParenthesisClose,
            Token::Identifier("gx".to_string()),
            Token::ParenthesisClose,
            Token::EndSetup,
            Token::EndOfFile,
        ];

        if tokens != expected_tokens {
            debug_tokens(&tokens, &expected_tokens);
        }
        
        assert_eq!(tokens, expected_tokens);
    }
}