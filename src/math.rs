use crate::expression::Expression;

pub trait FnDerivable {
    fn derive(&mut self, inner: Expression) -> Expression;
}

pub trait Derivable {
    fn derive(&mut self) -> Self;
}

pub trait Integrable {
    fn integrate(&mut self) -> Self;
}

pub trait EqInfo {
    fn has_variable(&self) -> bool;
}

pub trait Simplify {
    fn simplify(&self) -> Self;
}
