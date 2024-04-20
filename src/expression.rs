#[derive(Debug)]
pub struct Expression(pub ExpressionNode);

#[derive(Debug, Clone)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Exp,
}

#[derive(Debug, Clone)]
pub enum ExpressionToken {
    Number(f32),
    Operator(Operator),
    LParen,
    RParen,
    Variable(char),
    Node(ExpressionNode),
}

#[derive(Debug, Clone)]
pub enum ExpressionNode {
    Number(f32),
    Variable(char),
    Add(Box<ExpressionNode>, Box<ExpressionNode>),
    Sub(Box<ExpressionNode>, Box<ExpressionNode>),
    Mul(Box<ExpressionNode>, Box<ExpressionNode>),
    Div(Box<ExpressionNode>, Box<ExpressionNode>),
    Exp(Box<ExpressionNode>, Box<ExpressionNode>),
}
