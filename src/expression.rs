use std::ops::{Add, Div, Mul, Sub};

use crate::{operator::Operator, Constant};

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
