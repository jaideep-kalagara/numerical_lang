use numerical_lang::parser::{binary_op::BinaryOp, expr::Expression};

use crate::common::assert_pratt_parser;

pub mod common;

#[test]
fn basic_parser_test() {
    assert_pratt_parser(
        "1 + 2 * 3 / 2",
        Expression::Binary {
            lhs: Box::new(Expression::IntValue(1)),
            operator: BinaryOp::Add,
            rhs: Box::new(Expression::Binary {
                lhs: Box::new(Expression::Binary {
                    lhs: Box::new(Expression::IntValue(2)),
                    operator: BinaryOp::Mul,
                    rhs: Box::new(Expression::IntValue(3)),
                }),
                operator: BinaryOp::Div,
                rhs: Box::new(Expression::IntValue(2)),
            }),
        },
    );
}

#[test]
fn unary_parser_test() {
    assert_pratt_parser(
        "-1",
        Expression::Unary {
            operator: BinaryOp::Sub,
            rhs: Box::new(Expression::IntValue(1)),
        },
    );
}

#[test]
fn parenthesis_parser_test() {
    assert_pratt_parser(
        "2 * (1 + 10)",
        Expression::Binary {
            lhs: Box::new(Expression::IntValue(2)),
            operator: BinaryOp::Mul,
            rhs: Box::new(Expression::Binary {
                lhs: Box::new(Expression::IntValue(1)),
                operator: BinaryOp::Add,
                rhs: Box::new(Expression::IntValue(10)),
            }),
        },
    );
}
