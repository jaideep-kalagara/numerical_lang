use std::fmt::Display;

use crate::parser::binary_op::BinaryOp;

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    IntValue(i64),
    FloatValue(f64),
    Binary {
        lhs: Box<Expression>,
        operator: BinaryOp,
        rhs: Box<Expression>,
    },
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::IntValue(v) => write!(f, "{}", v),
            Expression::FloatValue(v) => write!(f, "{}", v),
            Expression::Binary { lhs, operator, rhs } => {
                write!(f, "({}", operator)?;
                write!(f, " {}", lhs)?;
                write!(f, " {})", rhs)?;

                Ok(())
            }
        }
    }
}
