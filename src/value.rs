use std::ops::Neg;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Value(pub f64);

impl Neg for Value {
    type Output = Value;

    fn neg(self) -> Self::Output {
        Value(-self.0)
    }
}
