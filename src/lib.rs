use std::fmt::Display;

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

impl Display for Constant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Constant::Euler => "e",
            Constant::Pi => "π",
            Constant::Num(val) => &val,
        };
        write!(f, "{}", value)
    }
}
