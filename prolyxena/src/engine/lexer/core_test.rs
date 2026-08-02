use crate::engine::lexer::core::*;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_engine_lexer_core_skip_whitespace_comments() {
        let content = "#test  ";
        let mut data = Lexer::new(content, String::from("path.nix"));
        data.skip_whitespace();

        assert!(data.chars.peek().is_none());
    }

    #[test]
    fn test_engine_lexer_core_skip_whitespace_whitespace() {
        let content1 = "  ";
        let content2 = "  t";

        let mut data1 = Lexer::new(content1, String::from("path.nix"));
        let mut data2 = Lexer::new(content2, String::from("path.nix"));

        data1.skip_whitespace();
        data2.skip_whitespace();

        assert!(data1.chars.peek().is_none());
        assert!(data2.chars.peek().is_some());
    }

    #[test]
    fn test_engine_lexer_core_parse_value_break_character() {
        let content1 = ";t";
        let content2 = ";";
        let content3 = "}";
        let content4 = "]";
        let content5 = ")";
        let content6 = "=";
        let content7 = "";

        let mut data1 = Lexer::new(content1, String::from("path.nix"));
        let mut data2 = Lexer::new(content2, String::from("path.nix"));
        let mut data3 = Lexer::new(content3, String::from("path.nix"));
        let mut data4 = Lexer::new(content4, String::from("path.nix"));
        let mut data5 = Lexer::new(content5, String::from("path.nix"));
        let mut data6 = Lexer::new(content6, String::from("path.nix"));
        let mut data7 = Lexer::new(content7, String::from("path.nix"));

        let result1 = data1.parse_value();
        let result2 = data2.parse_value();
        let result3 = data3.parse_value();
        let result4 = data4.parse_value();
        let result5 = data5.parse_value();
        let result6 = data6.parse_value();
        let result7 = data7.parse_value();

        assert!(data1.chars.peek().is_some());
        assert!(data2.chars.peek().is_some());
        assert!(data3.chars.peek().is_some());
        assert!(data4.chars.peek().is_some());
        assert!(data5.chars.peek().is_some());
        assert!(data6.chars.peek().is_some());
        assert!(data7.chars.peek().is_none());

        assert!(result1.is_err());
        assert!(result2.is_err());
        assert!(result3.is_err());
        assert!(result4.is_err());
        assert!(result5.is_err());
        assert!(result6.is_err());
        assert!(result7.is_err());

        data1.chars.next();
        data2.chars.next();
        data3.chars.next();
        data4.chars.next();
        data5.chars.next();
        data6.chars.next();

        assert!(data1.chars.peek().is_some());
        assert!(data2.chars.peek().is_none());
        assert!(data3.chars.peek().is_none());
        assert!(data4.chars.peek().is_none());
        assert!(data5.chars.peek().is_none());
        assert!(data6.chars.peek().is_none());

        data1.chars.next();

        assert!(data1.chars.peek().is_none());
    }
}
