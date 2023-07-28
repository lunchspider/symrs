#[cfg(test)]
mod tests {
    use crate::{lexer::lexer::Lexer, lexer::lexer::Token};
    #[test]
    fn lexer_bracket_test() -> anyhow::Result<()> {
        let left_parem = "(".to_string();
        let right_parem = ")".to_string();
        
        let left_squirly = "{".to_string();
        let right_squirly = "}".to_string();
        
        
        let left_parem_token = Lexer::new(left_parem).peekable().next().unwrap();
        let right_parem_token = Lexer::new(right_parem).peekable().next().unwrap();
        let left_squirly_token = Lexer::new(left_squirly).peekable().next().unwrap();
        let right_squirly_token = Lexer::new(right_squirly).peekable().next().unwrap();

        assert_eq!(left_parem_token, Token::LParem);
        assert_eq!(right_parem_token, Token::RParem);
        assert_eq!(left_squirly_token, Token::LSquirly);
        assert_eq!(right_squirly_token, Token::RSquirly);
        
        assert!(left_parem_token.is_bracket());
        assert!(right_parem_token.is_bracket());
        assert!(left_squirly_token.is_bracket());
        assert!(right_squirly_token.is_bracket());

        Ok(())
    }

    #[test]
    fn lexer_constant_test() -> anyhow::Result<()> {
        let int_zero = 0.to_string();
        let float_zero = (0.0).to_string();

        let int_constant = 1.to_string();
        let float_constant = (1.0).to_string();

        let int_zero_token = Lexer::new(int_zero).peekable().next().unwrap();
        let float_zero_token = Lexer::new(float_zero).peekable().next().unwrap();
        let int_constant_token = Lexer::new(int_constant).peekable().next().unwrap();
        let float_constant_token = Lexer::new(float_constant).peekable().next().unwrap();

        assert!(int_zero_token.is_constant());
        assert!(float_zero_token.is_constant());
        assert!(int_constant_token.is_constant());
        assert!(float_constant_token.is_constant());

        Ok(())
    }

    #[test]
    fn lexer_relational_operator_test() -> anyhow::Result<()> {
        let less_than = "<".to_string();
        let greater_than = ">".to_string();

        let less_than_equal_to = "<=".to_string();
        let greater_than_equal_to = ">=".to_string();


        let less_than_token = Lexer::new(less_than).peekable().next().unwrap();
        let greater_than_token = Lexer::new(greater_than).peekable().next().unwrap();
        let less_than_equal_to_token = Lexer::new(less_than_equal_to).peekable().next().unwrap();
        let greater_than_equal_to_token = Lexer::new(greater_than_equal_to).peekable().next().unwrap();

        assert_eq!(less_than_token, Token::LessThan);
        assert_eq!(greater_than_token, Token::GreaterThan);
        assert_eq!(less_than_equal_to_token, Token::LessThanEqualTo);
        assert_eq!(greater_than_equal_to_token, Token::GreaterThanEqualTo);

        assert!(less_than_token.is_relational());
        assert!(greater_than_token.is_relational());
        assert!(less_than_equal_to_token.is_relational());
        assert!(greater_than_equal_to_token.is_relational());

        Ok(())

    }
}
