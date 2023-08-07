#[cfg(test)]
mod tests {
    use crate::{error::SyntaxError, expression::Expression, lexer::lexer::Lexer, parser::Parser};

    fn parse_input(input: String) -> Result<Expression, SyntaxError> {
        let lexer = Lexer::new(input);
        let mut iter = lexer.peekable();
        let mut parser = Parser::new(&mut iter);

        println!("Parser Created");

        parser.parse()
    }

    fn panic_if_result_is_not_err(exp: Result<Expression, SyntaxError>) {
        match exp {
            Ok(exp1) => {
                dbg!(exp1);
                panic!()
            }
            Err(e) => println!("{}", e),
        }
    }

    #[test]
    fn parser_test_equal() -> anyhow::Result<()> {
        let input = "x = 2".to_string();

        let exp1 = parse_input(input)?;
        dbg!(exp1);
        Ok(())
    }

    #[test]
    fn parser_test_less_than() -> anyhow::Result<()> {
        let input = "x < 2".to_string();

        let exp1 = parse_input(input)?;
        dbg!(exp1);
        Ok(())
    }

    #[test]
    fn parser_test_less_than_equal_to() -> anyhow::Result<()> {
        let input = "x <= 2".to_string();

        let exp1 = parse_input(input)?;
        dbg!(exp1);
        Ok(())
    }

    #[test]
    fn parser_test_greater_than() -> anyhow::Result<()> {
        let input = "x > 2".to_string();

        let exp1 = parse_input(input)?;
        dbg!(exp1);
        Ok(())
    }

    #[test]
    fn parser_test_greater_than_equal_to() -> anyhow::Result<()> {
        let input = "x >= 2".to_string();

        let exp1 = parse_input(input)?;
        dbg!(exp1);
        Ok(())
    }

    #[test]
    fn parser_test_less_than_two_variables() -> anyhow::Result<()> {
        let input = "x < y".to_string();

        let exp1 = parse_input(input)?;
        dbg!(exp1);
        Ok(())
    }

    #[test]
    fn parser_test_variable_expression_1() -> anyhow::Result<()> {
        let input = "x".to_string();

        let exp1 = parse_input(input)?;
        dbg!(exp1);
        Ok(())
    }

    #[test]
    fn parser_test_variable_expression_2() -> anyhow::Result<()> {
        let input = "(x)".to_string();

        let exp1 = parse_input(input)?;
        dbg!(exp1);
        Ok(())
    }

    #[test]
    fn parser_test_variable_expression_3() -> anyhow::Result<()> {
        let input = "[x]".to_string();

        let exp1 = parse_input(input)?;
        dbg!(exp1);
        Ok(())
    }

    #[test]
    fn parser_test_variable_expression_4() -> anyhow::Result<()> {
        let input = "{x}".to_string();

        let exp1 = parse_input(input)?;
        dbg!(exp1);
        Ok(())
    }

    #[test]
    fn parser_test_linear_equation_1() -> anyhow::Result<()> {
        let input = "x + y = 3".to_string();

        let exp1 = parse_input(input)?;
        dbg!(exp1);
        Ok(())
    }

    #[test]
    fn parser_test_linear_equation_2() -> anyhow::Result<()> {
        let input = "3 * x + 4 * y = 5".to_string();

        let exp1 = parse_input(input)?;
        dbg!(exp1);
        Ok(())
    }

    #[test]
    fn parser_test_quadratic_equation_1() -> anyhow::Result<()> {
        let input = "3 * x ^ 2 + 4 * x = 5".to_string();

        let exp1 = parse_input(input)?;
        dbg!(exp1);
        Ok(())
    }

    #[test]
    fn parser_test_variable_power_equation_1() -> anyhow::Result<()> {
        let input = "3 * x ^ x + 4 * x ^ y = 5".to_string();

        let exp1 = parse_input(input)?;
        dbg!(exp1);
        Ok(())
    }

    #[test]
    fn parser_test_expression_equation_1() -> anyhow::Result<()> {
        let input = "3 / 5 * 2 + 8 * 7 >= 56".to_string();

        let exp1 = parse_input(input)?;
        dbg!(exp1);
        Ok(())
    }

    #[test]
    fn parser_test_expression_equation_2() -> anyhow::Result<()> {
        let input = "3 / (5 * 2) + 8 * 7 >= 56".to_string();

        let exp1 = parse_input(input)?;
        dbg!(exp1);
        Ok(())
    }

    #[test]
    fn parser_test_expression_equation_3() -> anyhow::Result<()> {
        let input = "3 / ((5 * 2) + 8) * 7 >= 56".to_string();

        let exp1 = parse_input(input)?;
        dbg!(exp1);
        Ok(())
    }

    #[test]
    pub fn parser_test_invalid_expression_01() -> anyhow::Result<()> {
        let input = "()".to_string();

        let exp1 = parse_input(input);
        panic_if_result_is_not_err(exp1);

        Ok(())
    }

    #[test]
    fn parser_test_invalid_expression_02() -> anyhow::Result<()> {
        let input = ")".to_string();

        let exp1 = parse_input(input);
        panic_if_result_is_not_err(exp1);

        Ok(())
    }

    #[test]
    fn parser_test_invalid_expression_03() -> anyhow::Result<()> {
        let input = "(".to_string();

        let exp1 = parse_input(input);
        panic_if_result_is_not_err(exp1);

        Ok(())
    }

    #[test]
    fn parser_test_invalid_expression_04() -> anyhow::Result<()> {
        let input = "[".to_string();

        let exp1 = parse_input(input);
        panic_if_result_is_not_err(exp1);

        Ok(())
    }

    #[test]
    fn parser_test_invalid_expression_05() -> anyhow::Result<()> {
        let input = "]".to_string();

        let exp1 = parse_input(input);
        panic_if_result_is_not_err(exp1);

        Ok(())
    }

    #[test]
    fn parser_test_invalid_expression_06() -> anyhow::Result<()> {
        let input = "[]".to_string();

        let exp1 = parse_input(input);
        panic_if_result_is_not_err(exp1);

        Ok(())
    }

    #[test]
    fn parser_test_invalid_expression_07() -> anyhow::Result<()> {
        let input = "[()]".to_string();

        let exp1 = parse_input(input);
        panic_if_result_is_not_err(exp1);

        Ok(())
    }

    #[test]
    fn parser_test_invalid_expression_08() -> anyhow::Result<()> {
        let input = "([])".to_string();

        let exp1 = parse_input(input);
        panic_if_result_is_not_err(exp1);

        Ok(())
    }

    #[test]
    fn parser_test_invalid_expression_09() -> anyhow::Result<()> {
        let input = "([)]".to_string();

        let exp1 = parse_input(input);
        panic_if_result_is_not_err(exp1);

        Ok(())
    }

    #[test]
    fn parser_test_invalid_expression_10() -> anyhow::Result<()> {
        let input = "([x)]".to_string();

        let exp1 = parse_input(input);
        panic_if_result_is_not_err(exp1);

        Ok(())
    }

    #[test]
    fn parser_test_invalid_expression_11() -> anyhow::Result<()> {
        let input = "{".to_string();

        let exp1 = parse_input(input);
        panic_if_result_is_not_err(exp1);

        Ok(())
    }

    #[test]
    fn parser_test_invalid_expression_12() -> anyhow::Result<()> {
        let input = "}".to_string();

        let exp1 = parse_input(input);
        panic_if_result_is_not_err(exp1);

        Ok(())
    }

    #[test]
    fn parser_test_invalid_expression_13() -> anyhow::Result<()> {
        let input = "{}".to_string();

        let exp1 = parse_input(input);
        panic_if_result_is_not_err(exp1);

        Ok(())
    }

    #[test]
    fn parser_test_invalid_expression_14() -> anyhow::Result<()> {
        let input = "{()}".to_string();

        let exp1 = parse_input(input);
        panic_if_result_is_not_err(exp1);

        Ok(())
    }

    #[test]
    fn parser_test_invalid_expression_15() -> anyhow::Result<()> {
        let input = "({})".to_string();

        let exp1 = parse_input(input);
        panic_if_result_is_not_err(exp1);

        Ok(())
    }

    #[test]
    fn parser_test_invalid_expression_16() -> anyhow::Result<()> {
        let input = "({)}".to_string();

        let exp1 = parse_input(input);
        panic_if_result_is_not_err(exp1);

        Ok(())
    }

    #[test]
    fn parser_test_invalid_expression_17() -> anyhow::Result<()> {
        let input = "({x)}".to_string();

        let exp1 = parse_input(input);
        panic_if_result_is_not_err(exp1);

        Ok(())
    }
}
