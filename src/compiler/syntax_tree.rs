use crate::compiler::scanner::Token;
use std::fmt;
use std::fmt::{Error, Formatter, Write};

#[derive(Debug, Clone, Copy, PartialEq)]
enum UnaryOperator {
    Minus,
    Bang,
}

impl fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let op_str = match self {
            UnaryOperator::Minus => "-",
            UnaryOperator::Bang => "!",
        };
        write!(f, "{}", op_str)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum BinaryOperator {
    EqualEqual,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Plus,
    Minus,
    Star,
    Slash,
}

impl fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let op_str = match self {
            BinaryOperator::EqualEqual => "==",
            BinaryOperator::BangEqual => "!=",
            BinaryOperator::Less => "<",
            BinaryOperator::LessEqual => "<=",
            BinaryOperator::Greater => ">",
            BinaryOperator::GreaterEqual => ">=",
            BinaryOperator::Plus => "+",
            BinaryOperator::Minus => "-",
            BinaryOperator::Star => "*",
            BinaryOperator::Slash => "/",
        };
        write!(f, "{}", op_str)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Literal<'a> {
    Identifier(&'a str),
    String(&'a str),
    Number(f64),
    Nil,
}

impl<'a> fmt::Display for Literal<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Identifier(s) => write!(f, "{}", s),
            Literal::String(s) => write!(f, "{}", s),
            Literal::Number(n) => write!(f, "{}", n),
            Literal::Nil => write!(f, "nil"),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Expression<'a> {
    Binary {
        left: Box<Expression<'a>>,
        operator: BinaryOperator,
        right: Box<Expression<'a>>,
    },
    Grouping {
        expression: Box<Expression<'a>>,
    },
    Literal {
        value: Literal<'a>,
    },
    Unary {
        operator: UnaryOperator,
        right: Box<Expression<'a>>,
    },
}

impl fmt::Display for Expression<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Binary {
                left,
                operator,
                right,
            } => write!(f, "({operator} {left} {right})"),
            Expression::Grouping { expression } => write!(f, "(group {expression})"),
            Expression::Literal { value } => write!(f, "{value}"),
            Expression::Unary { operator, right } => write!(f, "({operator} {right})"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expression_display() {
        let expr = Expression::Binary {
            left: Box::new(Expression::Unary {
                operator: UnaryOperator::Minus,
                right: Box::new(Expression::Literal {
                    value: Literal::Number(123f64),
                }),
            }),
            operator: BinaryOperator::Star,
            right: Box::new(Expression::Grouping {
                expression: Box::new(Expression::Literal {
                    value: Literal::Number(45.67),
                }),
            }),
        };

        let s = format!("{expr}");
        assert_eq!(s, "(* (- 123) (group 45.67))")
    }
}
