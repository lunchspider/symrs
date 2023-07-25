use std::iter::Peekable;

use crate::error::SyntaxError;
use crate::{lexer::lexer::Token, Expression, Operator};
use anyhow::Result;

pub struct Parser<'a, T>
where
    T: Iterator<Item = Token>,
{
    iter: &'a mut Peekable<T>,
}

impl<'a, T> Parser<'a, T>
where
    T: Iterator<Item = Token>,
{
    pub fn new(iter: &'a mut Peekable<T>) -> Self {
        Parser { iter }
    }

    fn assert_next(&mut self, token: Token) -> Result<(), SyntaxError> {
        let next = self.iter.next();
        match next {
            Some(next) if next != token => Err(SyntaxError::new_parse_error(format!(
                "Expected {:?} actual {:?}",
                token, next,
            ))),
            None => Err(SyntaxError::new_parse_error(
                "Unexpected end of input".to_string(),
            )),
            _ => Ok(()),
        }
    }

    fn primary(&mut self) -> Result<Expression, SyntaxError> {
        match self.iter.next().unwrap() {
            Token::Minus => {
                let op = Operator::Negative;
                let expr = self.expression(op.cmp_val())?;
                Ok(Expression::Unary(op, Box::new(expr)))
            }
            Token::LParem => {
                let expr = self.expression(0)?;
                self.assert_next(Token::RParem)?;
                Ok(expr)
            }
            Token::Constant(n) => Ok(Expression::Constant(n.clone())),
            Token::Symbol(x) => Ok(Expression::Symbol(x.clone())),
            tok => Err(SyntaxError::new_parse_error(format!(
                "Unexpected token {:?}",
                tok
            ))),
        }
    }

    fn expression(&mut self, precedence: usize) -> Result<Expression, SyntaxError> {
        let mut expr = self.primary()?;
        while let Some(tok) = self.iter.peek() {
            if !tok.is_binary() {
                break;
            }
            let operator = Operator::try_from(tok).unwrap();
            if operator.cmp_val() < precedence {
                break;
            }
            self.iter.next();
            let inner_precedence = match operator {
                Operator::Power => operator.cmp_val(),
                _ => 1 + operator.cmp_val(),
            };
            let rhs = self.expression(inner_precedence)?;
            expr = Expression::Binary(operator, Box::new(expr), Box::new(rhs));
        }

        Ok(expr)
    }

    pub fn parse(&mut self) -> Result<Expression, SyntaxError> {
        let ast = self.expression(0)?;
        self.assert_next(Token::Eof)?;
        Ok(ast)
    }
}
