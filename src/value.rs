use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Sub},
};

use crate::bytecode::core::BinaryOp;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Value(pub f64);

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Value {
    pub fn apply_binary_op(a: Value, b: Value, op: BinaryOp) -> Value {
        match op {
            BinaryOp::Add => a + b,
            BinaryOp::Subtract => a - b,
            BinaryOp::Multiply => a * b,
            BinaryOp::Divide => a / b,
        }
    }
}

impl Neg for Value {
    type Output = Value;

    fn neg(self) -> Self::Output {
        Value(-self.0)
    }
}

impl Add for Value {
    type Output = Value;

    fn add(self, rhs: Self) -> Self::Output {
        Value(self.0 + rhs.0)
    }
}

impl Sub for Value {
    type Output = Value;

    fn sub(self, rhs: Self) -> Self::Output {
        Value(self.0 - rhs.0)
    }
}

impl Mul for Value {
    type Output = Value;

    fn mul(self, rhs: Self) -> Self::Output {
        Value(self.0 * rhs.0)
    }
}

impl Div for Value {
    type Output = Value;

    fn div(self, rhs: Self) -> Self::Output {
        Value(self.0 / rhs.0)
    }
}
