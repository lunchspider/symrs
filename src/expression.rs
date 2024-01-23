use crate::{operator::Operator, Constant};
use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Expression {
    Constant(Constant),
    Binary(Operator, Box<Expression>, Box<Expression>),
    Unary(Operator, Box<Expression>),
    LessThan(Box<Self>, Box<Self>),
    GreaterThan(Box<Self>, Box<Self>),
    Equal(Box<Self>, Box<Self>),
    Variable(String),
}

impl Expression {
    fn is_binary(&self) -> bool {
        match self {
            Expression::Binary(_, _, _) => true,
            _ => false,
        }
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Constant(x) => {
                write!(f, "{}", x)
            }
            Expression::Binary(op, lhs, rhs) => {           
                if lhs.is_binary() {
                    write!(f, "({})", lhs)?;
                } else {
                    write!(f, "{} ", lhs)?;
                }
                    write!(f, "{} ", op)?;
                if rhs.is_binary() {
                    write!(f, "({})", rhs)?;
                } else {
                    write!(f, "{}", rhs)?;
                }
                Ok(())
            },
            Expression::Unary(op, expr) => match **expr {
                Expression::Binary(_, _, _) => {
                    write!(f, "{} ({})", op, expr)
                }
                _ => {
                    write!(f, "{} {}", op, expr)
                }
            },
            Expression::LessThan(lhs, rhs) => write!(f, "{} < {}", lhs, rhs),
            Expression::GreaterThan(lhs, rhs) => write!(f, "{} > {}", lhs, rhs),
            Expression::Equal(lhs, rhs) => write!(f, "{} = {}", lhs, rhs),
            Expression::Variable(x) => {
                write!(f, "{}", x)
            }
        }
    }
}

impl Add for Expression {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Expression::Binary(Operator::Add, Box::new(self), Box::new(rhs))
    }
}

impl Sub for Expression {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Expression::Binary(Operator::Subtract, Box::new(self), Box::new(rhs))
    }
}

impl Mul for Expression {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Expression::Binary(Operator::Multiply, Box::new(self), Box::new(rhs))
    }
}

impl Div for Expression {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Expression::Binary(Operator::Divide, Box::new(self), Box::new(rhs))
    }
}
