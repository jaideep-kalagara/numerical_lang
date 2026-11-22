use crate::parser::binary_op::BinaryOp;

pub mod binary_op;

pub enum Expression {
    IntValue(i64),
    Binary {
        lhs: Box<Expression>,
        operator: BinaryOp,
        rhs: Box<Expression>,
    },
}
