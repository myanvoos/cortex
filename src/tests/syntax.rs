#[cfg(test)]
mod test {
    use crate::parser::extracted_string_content;
    use super::*;

    #[test]
    fn test_string_extraction() {
        let input1 = "documentclass(\"article\")";
        let input2 = "documentclass('article')";
        let input3 = "documentclass('')";
        let input4 = "documentclass()";

        assert_eq!(extracted_string_content(&input1), Some("article".to_string()));
        assert_eq!(extracted_string_content(&input2), Some("article".to_string()));
        assert_eq!(extracted_string_content(&input3), Some("".to_string()));
        assert_eq!(extracted_string_content(&input4), None);
        assert_eq!(
            extracted_string_content(r#"documentclass("article")"#),
            Some("article".to_string())
        );
    
    }
}