use crate::math::{Derivable, EqInfo, FnDerivable, Simplify};

#[derive(Debug, Clone)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Exp,
}

#[derive(Debug, Clone)]
pub enum FunctionName {
    Cos,
    Sin,
    Tan,
    Sec,
    Csc,
    Cot,
    ArcCos,
    ArcSin,
    ArcTan,
    ArcSec,
    ArcCsc,
    ArcCot,
    Ln,
    Log,
    Abs,
    Floor,
}

impl FnDerivable for FunctionName {
    fn derive(&mut self, mut inner: Expression) -> Expression {
        match self {
            FunctionName::Cos => {
                let dx_inner = inner.derive().simplify();
                let res = Expression::Mul(
                    Box::new(Expression::Mul(
                        Box::new(Expression::Number(-1.0)),
                        Box::new(dx_inner),
                    )),
                    Box::new(Expression::Function(FunctionName::Sin, Box::new(inner))),
                );

                res.simplify()
            }
            FunctionName::Sin => {
                let dx_inner = inner.derive().simplify();
                let res = Expression::Mul(
                    Box::new(dx_inner),
                    Box::new(Expression::Function(FunctionName::Cos, Box::new(inner))),
                );

                res.simplify()
            }
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ExpressionToken {
    Number(f32),
    Operator(Operator),
    LParen,
    RParen,
    Variable(String, Expression),
    Function(FunctionName),
    Node(Expression),
}

#[derive(Debug, Clone)]
pub enum Expression {
    Number(f32),
    Variable(String, Box<Expression>),
    Function(FunctionName, Box<Expression>),
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),
    Exp(Box<Expression>, Box<Expression>),
}

impl EqInfo for Expression {
    fn has_variable(&self) -> bool {
        match self {
            Expression::Number(_) => false,
            Expression::Add(left, right) => left.has_variable() || right.has_variable(),
            Expression::Sub(left, right) => left.has_variable() || right.has_variable(),
            Expression::Mul(left, right) => left.has_variable() || right.has_variable(),
            Expression::Div(left, right) => left.has_variable() || right.has_variable(),
            Expression::Variable(_, _) => true,
            Expression::Exp(base, exp) => base.has_variable() || exp.has_variable(),
            _ => unimplemented!(),
        }
    }
}

impl Simplify for Expression {
    fn simplify(&self) -> Self {
        match self {
            Expression::Add(left, right) => {
                let left = left.simplify();
                let right = right.simplify();
                let mut res = Expression::Add(Box::new(left.clone()), Box::new(right.clone()));

                if let Expression::Number(num) = left {
                    if let Expression::Number(right_num) = right {
                        res = Expression::Number(num + right_num);
                    } else if num == 0.0 {
                        res = right.clone();
                    }
                } else if let Expression::Number(num) = right {
                    if num == 0.0 {
                        res = left.clone();
                    }
                }

                res
            }
            Expression::Sub(left, right) => {
                let left = left.simplify();
                let right = right.simplify();
                let mut res = Expression::Sub(Box::new(left.clone()), Box::new(right.clone()));

                if let Expression::Number(num) = left {
                    if let Expression::Number(right_num) = right {
                        res = Expression::Number(num - right_num);
                    } else if num == 0.0 {
                        let temp = Expression::Mul(
                            Box::new(Expression::Number(-1.0)),
                            Box::new(right.clone()),
                        );
                        res = temp.simplify();
                    }
                } else if let Expression::Number(num) = right {
                    if num == 0.0 {
                        res = left.clone();
                    }
                }

                res
            }
            Expression::Mul(left, right) => {
                let left = left.simplify();
                let right = right.simplify();
                let mut res = Expression::Sub(Box::new(left.clone()), Box::new(right.clone()));

                if let Expression::Number(num) = left {
                    if num == 0.0 {
                        res = Expression::Number(0.0);
                    } else if num == 1.0 {
                        res = right.clone();
                    } else if num == -1.0 {
                        if let Expression::Number(right_num) = right {
                            res = Expression::Number(-right_num);
                        }
                    }
                } else if let Expression::Number(num) = right {
                    if num == 0.0 {
                        res = Expression::Number(0.0);
                    } else if num == 1.0 {
                        res = right.clone();
                    } else if num == -1.0 {
                        if let Expression::Number(left_num) = left {
                            res = Expression::Number(-left_num);
                        }
                    }
                }

                res
            }
            Expression::Div(upper, lower) => {
                let upper = upper.simplify();
                let lower = lower.simplify();
                let mut res = Expression::Div(Box::new(upper.clone()), Box::new(lower.clone()));

                if let Expression::Number(num) = lower {
                    if num == 1.0 {
                        res = upper;
                    } else if num == 0.0 {
                        panic!("Divide by 0");
                    }
                }

                res
            }
            Expression::Variable(name, exp) => {
                let exp = exp.simplify();

                let mut res = Expression::Variable(name.clone(), Box::new(exp.clone()));

                if let Expression::Number(num) = exp {
                    if num == 0.0 {
                        res = Expression::Number(1.0);
                    }
                }

                res
            }
            Expression::Number(n) => Expression::Number(*n),
            Expression::Function(func, inner) => {
                Expression::Function(func.clone(), Box::new(inner.simplify()))
            }
            Expression::Exp(base, exp) => {
                let base = base.simplify();
                let exp = exp.simplify();

                let mut res: Option<Expression> = None;

                if let Expression::Number(num) = base {
                    if num == 0.0 {
                        res = Some(Expression::Number(0.0));
                    } else if num == 1.0 {
                        res = Some(Expression::Number(1.0));
                    }
                }

                if res.is_none() {
                    if let Expression::Number(num) = exp {
                        if num == 0.0 {
                            res = Some(Expression::Number(1.0));
                        } else if num == 1.0 {
                            res = Some(base.clone());
                        }
                    }
                }

                res.unwrap_or(Expression::Exp(Box::new(base), Box::new(exp)))
            }
        }
    }
}

impl Derivable for Expression {
    fn derive(&mut self) -> Self {
        match self {
            Expression::Add(left, right) => {
                *left.as_mut() = left.derive();
                *right.as_mut() = right.derive();

                let res = Expression::Add(left.clone(), right.clone());
                res.simplify()
            }
            Expression::Sub(left, right) => {
                *left.as_mut() = left.derive();
                *right.as_mut() = right.derive();

                let res = Expression::Add(left.clone(), right.clone());
                res.simplify()
            }
            Expression::Number(_) => Expression::Number(0.0),
            Expression::Variable(name, exp) => {
                let res = if exp.has_variable() {
                    unimplemented!()
                } else {
                    Expression::Mul(
                        Box::new(*exp.clone()),
                        Box::new(Expression::Variable(
                            name.clone(),
                            Box::new(Expression::Sub(
                                Box::new(*exp.clone()),
                                Box::new(Expression::Number(1.0)),
                            )),
                        )),
                    )
                };

                res.simplify()
            }
            Expression::Function(func, inner) => func.derive(*inner.clone()).simplify(),
            _ => unimplemented!(),
        }
    }
}
