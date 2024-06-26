use crate::expression::Expression;

pub trait FnDerivable {
    fn derive(&self, inner: Expression) -> Expression;
}

pub trait Derivable {
    fn derive(&self) -> Self;
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
