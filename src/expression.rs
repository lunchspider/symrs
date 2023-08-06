use std::{collections::HashMap, ops::Add};

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

//impl Expression {
//    pub fn get_coefficients(&self) -> HashMap<String, Expression> {}
//}

impl Add for Expression {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}
