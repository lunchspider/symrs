#[cfg(test)]
mod tests {
    use crate::{lexer::lexer::Lexer, parser::Parser};
    #[test]
    fn parser_test() -> anyhow::Result<()> {
        let input = "x".to_string();
        let lexer = Lexer::new(input);
        let mut iter = lexer.peekable();
        let mut parser = Parser::new(&mut iter);
        let exp1 = parser.parse()?;
        dbg!(exp1);
        Ok(())
    }
}
