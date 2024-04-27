use std::fmt::Display;

use crate::math::{Derivable, EqInfo, FnDerivable, Simplify};

#[derive(Debug, Clone)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Exp,
}

#[derive(Debug, Clone, PartialEq)]
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
}

impl Display for FunctionName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            FunctionName::Cos => "cos",
            FunctionName::Sin => "sin",
            FunctionName::Tan => "tan",
            FunctionName::Sec => "sec",
            FunctionName::Csc => "csc",
            FunctionName::Cot => "cot",
            FunctionName::ArcCos => "arccos",
            FunctionName::ArcSin => "arcsin",
            FunctionName::ArcTan => "arctan",
            FunctionName::ArcSec => "arcsec",
            FunctionName::ArcCsc => "arccsc",
            FunctionName::ArcCot => "arccot",
            FunctionName::Ln => "ln",
            FunctionName::Log => "log",
            FunctionName::Abs => "abs",
        };

        write!(f, "{}", res)
    }
}

impl FnDerivable for FunctionName {
    fn derive(&self, inner: Expression) -> Expression {
        let dx_inner = inner.derive().simplify();

        match self {
            FunctionName::Cos => {
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
                let res = Expression::Mul(
                    Box::new(dx_inner),
                    Box::new(Expression::Function(FunctionName::Cos, Box::new(inner))),
                );

                res.simplify()
            }
            FunctionName::Tan => {
                let res = Expression::Mul(
                    Box::new(dx_inner),
                    Box::new(Expression::Exp(
                        Box::new(Expression::Function(FunctionName::Sec, Box::new(inner))),
                        Box::new(Expression::Number(2.0)),
                    )),
                );

                res.simplify()
            }
            FunctionName::Sec => {
                let res = Expression::Mul(
                    Box::new(dx_inner),
                    Box::new(Expression::Mul(
                        Box::new(Expression::Function(
                            FunctionName::Sec,
                            Box::new(inner.clone()),
                        )),
                        Box::new(Expression::Function(FunctionName::Tan, Box::new(inner))),
                    )),
                );

                res.simplify()
            }
            FunctionName::Csc => {
                let res = Expression::Mul(
                    Box::new(Expression::Number(-1.0)),
                    Box::new(Expression::Mul(
                        Box::new(dx_inner),
                        Box::new(Expression::Mul(
                            Box::new(Expression::Function(
                                FunctionName::Cot,
                                Box::new(inner.clone()),
                            )),
                            Box::new(Expression::Function(FunctionName::Csc, Box::new(inner))),
                        )),
                    )),
                );

                res.simplify()
            }
            FunctionName::Cot => {
                let res = Expression::Mul(
                    Box::new(dx_inner),
                    Box::new(Expression::Exp(
                        Box::new(Expression::Function(FunctionName::Csc, Box::new(inner))),
                        Box::new(Expression::Number(2.0)),
                    )),
                );

                res.simplify()
            }
            FunctionName::ArcCos => {
                let res = Expression::Mul(
                    Box::new(dx_inner),
                    Box::new(Expression::Div(
                        Box::new(Expression::Number(-1.0)),
                        Box::new(Expression::Exp(
                            Box::new(Expression::Sub(
                                Box::new(Expression::Number(1.0)),
                                Box::new(Expression::Exp(
                                    Box::new(inner),
                                    Box::new(Expression::Number(2.0)),
                                )),
                            )),
                            Box::new(Expression::Number(0.5)),
                        )),
                    )),
                );

                res.simplify()
            }
            FunctionName::ArcSin => {
                let res = Expression::Mul(
                    Box::new(dx_inner),
                    Box::new(Expression::Div(
                        Box::new(Expression::Number(1.0)),
                        Box::new(Expression::Exp(
                            Box::new(Expression::Sub(
                                Box::new(Expression::Number(1.0)),
                                Box::new(Expression::Exp(
                                    Box::new(inner),
                                    Box::new(Expression::Number(2.0)),
                                )),
                            )),
                            Box::new(Expression::Number(0.5)),
                        )),
                    )),
                );

                res.simplify()
            }
            FunctionName::ArcTan => {
                let res = Expression::Mul(
                    Box::new(dx_inner),
                    Box::new(Expression::Div(
                        Box::new(Expression::Number(1.0)),
                        Box::new(Expression::Add(
                            Box::new(Expression::Number(1.0)),
                            Box::new(Expression::Exp(
                                Box::new(inner),
                                Box::new(Expression::Number(2.0)),
                            )),
                        )),
                    )),
                );

                res.simplify()
            }
            FunctionName::ArcSec => {
                let res = Expression::Mul(
                    Box::new(dx_inner),
                    Box::new(Expression::Div(
                        Box::new(Expression::Number(1.0)),
                        Box::new(Expression::Mul(
                            Box::new(Expression::Function(
                                FunctionName::Abs,
                                Box::new(inner.clone()),
                            )),
                            Box::new(Expression::Exp(
                                Box::new(Expression::Sub(
                                    Box::new(Expression::Number(1.0)),
                                    Box::new(Expression::Exp(
                                        Box::new(inner),
                                        Box::new(Expression::Number(2.0)),
                                    )),
                                )),
                                Box::new(Expression::Number(0.5)),
                            )),
                        )),
                    )),
                );

                res.simplify()
            }
            FunctionName::ArcCsc => {
                let res = Expression::Mul(
                    Box::new(dx_inner),
                    Box::new(Expression::Div(
                        Box::new(Expression::Number(-1.0)),
                        Box::new(Expression::Mul(
                            Box::new(inner.clone()),
                            Box::new(Expression::Exp(
                                Box::new(Expression::Sub(
                                    Box::new(Expression::Exp(
                                        Box::new(inner),
                                        Box::new(Expression::Number(2.0)),
                                    )),
                                    Box::new(Expression::Number(1.0)),
                                )),
                                Box::new(Expression::Number(0.5)),
                            )),
                        )),
                    )),
                );

                res.simplify()
            }
            FunctionName::ArcCot => {
                let res = Expression::Mul(
                    Box::new(dx_inner),
                    Box::new(Expression::Div(
                        Box::new(Expression::Number(-1.0)),
                        Box::new(Expression::Add(
                            Box::new(Expression::Exp(
                                Box::new(inner),
                                Box::new(Expression::Number(2.0)),
                            )),
                            Box::new(Expression::Number(1.0)),
                        )),
                    )),
                );

                res.simplify()
            }
            FunctionName::Ln => {
                let res = Expression::Mul(
                    Box::new(dx_inner),
                    Box::new(Expression::Div(
                        Box::new(Expression::Number(1.0)),
                        Box::new(inner),
                    )),
                );

                res.simplify()
            }
            FunctionName::Log => {
                let res = Expression::Mul(
                    Box::new(dx_inner),
                    Box::new(Expression::Div(
                        Box::new(Expression::Number(1.0)),
                        Box::new(Expression::Mul(
                            Box::new(inner),
                            Box::new(Expression::Function(
                                FunctionName::Ln,
                                Box::new(Expression::Number(10.0)),
                            )),
                        )),
                    )),
                );

                res.simplify()
            }
            FunctionName::Abs => {
                let res = Expression::Mul(
                    Box::new(dx_inner),
                    Box::new(Expression::Div(
                        Box::new(inner.clone()),
                        Box::new(Expression::Function(FunctionName::Abs, Box::new(inner))),
                    )),
                );

                res.simplify()
            }
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

#[derive(Debug, Clone, PartialEq)]
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

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            Expression::Number(n) => n.to_string(),
            Expression::Variable(name, exp) => {
                let exp_str = exp.to_string();

                let mut res = name.to_owned() + "^" + exp_str.as_str();

                if let Expression::Number(num) = exp.as_ref() {
                    if *num == 1.0 {
                        res = name.clone();
                    }
                }

                res
            }
            Expression::Exp(base, exp) => {
                let base_str = base.to_string();
                let exp_str = exp.to_string();
                base_str.to_owned() + "^(" + exp_str.as_str() + ")"
            }
            Expression::Function(fn_name, inner) => {
                let inner_str = inner.to_string();
                fn_name.to_string() + "(" + inner_str.as_str() + ")"
            }
            Expression::Add(left, right) => {
                let left_str = left.to_string();
                let right_str = right.to_string();
                "(".to_owned() + left_str.as_str() + " + " + right_str.as_str() + ")"
            }
            Expression::Sub(left, right) => {
                let left_str = left.to_string();
                let right_str = right.to_string();
                "(".to_owned() + left_str.as_str() + " - " + right_str.as_str() + ")"
            }
            Expression::Mul(left, right) => {
                let left_str = left.to_string();
                let right_str = right.to_string();
                "(".to_owned() + left_str.as_str() + " * " + right_str.as_str() + ")"
            }
            Expression::Div(top, bottom) => {
                let top_str = top.to_string();
                let bottom_str = bottom.to_string();
                "(".to_owned() + top_str.as_str() + " / " + bottom_str.as_str() + ")"
            }
        };

        write!(f, "{}", res)
    }
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
            Expression::Function(_, inner) => inner.has_variable(),
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
                let mut res = Expression::Mul(Box::new(left.clone()), Box::new(right.clone()));

                if let Expression::Number(num) = left {
                    if let Expression::Number(right_num) = right {
                        res = Expression::Number(num * right_num);
                    } else if num == 0.0 {
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
                        res = left.clone();
                    } else if num == -1.0 {
                        if let Expression::Number(left_num) = left {
                            res = Expression::Number(-left_num);
                        }
                    }
                }

                if let Expression::Div(top, bottom) = right {
                    if let Expression::Number(num) = top.as_ref() {
                        if *num == 1.0 {
                            res = Expression::Div(Box::new(left.clone()), bottom.clone());
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
                } else if upper == lower {
                    res = Expression::Number(1.0);
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

                if let Expression::Variable(name, var_exp) = &base {
                    let temp = Expression::Mul(var_exp.clone(), Box::new(exp.clone()));
                    res = Some(Expression::Variable(
                        name.clone(),
                        Box::new(temp.simplify()),
                    ));
                }

                res.unwrap_or(Expression::Exp(Box::new(base), Box::new(exp)))
            }
        }
    }
}

impl Derivable for Expression {
    fn derive(&self) -> Self {
        match self {
            Expression::Add(left, right) => {
                let left = left.derive();
                let right = right.derive();

                let res = Expression::Add(Box::new(left), Box::new(right));
                res.simplify()
            }
            Expression::Sub(left, right) => {
                let left = left.derive();
                let right = right.derive();

                let res = Expression::Sub(Box::new(left), Box::new(right));
                res.simplify()
            }
            Expression::Mul(left, right) => {
                let left_derive = left.derive();
                let right_derive = right.derive();

                let res = Expression::Add(
                    Box::new(Expression::Mul(left.clone(), Box::new(right_derive))),
                    Box::new(Expression::Mul(right.clone(), Box::new(left_derive))),
                );
                res.simplify()
            }
            Expression::Div(top, bottom) => {
                let top_derive = top.derive();
                let bottom_derive = bottom.derive();

                let res = Expression::Div(
                    Box::new(Expression::Sub(
                        Box::new(Expression::Mul(bottom.clone(), Box::new(top_derive))),
                        Box::new(Expression::Mul(top.clone(), Box::new(bottom_derive))),
                    )),
                    Box::new(Expression::Exp(
                        bottom.clone(),
                        Box::new(Expression::Number(2.0)),
                    )),
                );
                res.simplify()
            }
            Expression::Exp(base, exp) => {
                let res = if exp.has_variable() {
                    if base.has_variable() {
                        let temp = Expression::Mul(
                            exp.clone(),
                            Box::new(Expression::Function(FunctionName::Ln, base.clone())),
                        );
                        let temp = temp.derive();

                        Expression::Mul(Box::new(self.clone()), Box::new(temp))
                    } else {
                        Expression::Mul(
                            Box::new(Expression::Function(FunctionName::Ln, base.clone())),
                            Box::new(self.clone()),
                        )
                    }
                } else {
                    let base_derive = base.derive();
                    Expression::Mul(
                        Box::new(base_derive),
                        Box::new(Expression::Sub(
                            exp.clone(),
                            Box::new(Expression::Number(1.0)),
                        )),
                    )
                };

                res.simplify()
            }
            Expression::Number(_) => Expression::Number(0.0),
            Expression::Variable(name, exp) => {
                let res = if exp.has_variable() {
                    let temp = Expression::Mul(
                        exp.clone(),
                        Box::new(Expression::Function(
                            FunctionName::Ln,
                            Box::new(Expression::Variable(
                                name.clone(),
                                Box::new(Expression::Number(1.0)),
                            )),
                        )),
                    );
                    let temp = temp.derive();

                    Expression::Mul(Box::new(self.clone()), Box::new(temp))
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
        }
    }
}
