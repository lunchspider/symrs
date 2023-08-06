pub mod error;
mod test;

pub mod expression;
pub mod lexer;
pub mod operator;
pub mod parser;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Constant {
    Euler,
    Pi,
    Num(String),
}
