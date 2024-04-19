pub struct Expression(ExpressionNode);

#[derive(Debug, Clone, Copy)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone, Copy)]
pub enum ExpressionToken {
    Number(f32),
    Operator(Operator),
    LParen,
    RParen,
}

pub enum ExpressionNode {
    Number(f32),
    Variable(char),
    Add(Box<ExpressionNode>, Box<ExpressionNode>),
    Sub(Box<ExpressionNode>, Box<ExpressionNode>),
    Mul(Box<ExpressionNode>, Box<ExpressionNode>),
    Div(Box<ExpressionNode>, Box<ExpressionNode>),
}
