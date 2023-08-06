#[cfg(test)]
mod tests {
    use crate::{lexer::lexer::Lexer, parser::Parser};

    #[test]
    fn parser_test_equal() -> anyhow::Result<()> {
        let input = "x = 2".to_string();
        let lexer = Lexer::new(input);
        let mut iter = lexer.peekable();
        let mut parser = Parser::new(&mut iter);
        
        println!("Parser Created");
        
        let exp1 = parser.parse()?;
        dbg!(exp1);
        Ok(())
    }
    
    #[test]
    fn parser_test_less_than() -> anyhow::Result<()> {
        let input = "x < 2".to_string();
        let lexer = Lexer::new(input);
        let mut iter = lexer.peekable();
        let mut parser = Parser::new(&mut iter);
        
        println!("Parser Created");
        
        let exp1 = parser.parse()?;
        dbg!(exp1);
        Ok(())
    }

    #[test]
    fn parser_test_less_than_equal_to() -> anyhow::Result<()> {
        let input = "x <= 2".to_string();
        let lexer = Lexer::new(input);
        let mut iter = lexer.peekable();
        let mut parser = Parser::new(&mut iter);
        
        println!("Parser Created");
        
        let exp1 = parser.parse()?;
        dbg!(exp1);
        Ok(())
    }

    #[test]
    fn parser_test_greater_than() -> anyhow::Result<()> {
        let input = "x > 2".to_string();
        let lexer = Lexer::new(input);
        let mut iter = lexer.peekable();
        let mut parser = Parser::new(&mut iter);
        
        println!("Parser Created");
        
        let exp1 = parser.parse()?;
        dbg!(exp1);
        Ok(())
    }

    #[test]
    fn parser_test_greater_than_equal_to() -> anyhow::Result<()> {
        let input = "x >= 2".to_string();
        let lexer = Lexer::new(input);
        let mut iter = lexer.peekable();
        let mut parser = Parser::new(&mut iter);
        
        println!("Parser Created");
        
        let exp1 = parser.parse()?;
        dbg!(exp1);
        Ok(())
    }

    #[test]
    fn parser_test_less_than_two_variables() -> anyhow::Result<()> {
        let input = "x < y".to_string();
        let lexer = Lexer::new(input);
        let mut iter = lexer.peekable();
        let mut parser = Parser::new(&mut iter);
        
        println!("Parser Created");
        
        let exp1 = parser.parse()?;
        dbg!(exp1);
        Ok(())
    }

    
}
