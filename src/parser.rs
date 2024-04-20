use crate::expression::{Expression, ExpressionNode, ExpressionToken, Operator};

pub fn parse(tokens: Vec<ExpressionToken>) -> Expression {
    Expression(parse_to_node(tokens))
}

macro_rules! create_groups {
    ($tokens:expr, $matches:pat, $not_matches:pat $(,)?) => {{
        let mut stage: Vec<ExpressionToken> = vec![];
        let mut i = 0;

        while i < $tokens.len() {
            let mut group: Vec<ExpressionToken> = vec![];
            let mut has_op = false;
            let mut diff = 0;

            while i + diff < $tokens.len()
                && match $tokens[i + diff] {
                    $not_matches => false,
                    _ => true,
                }
            {
                if matches!($tokens[i + diff], $matches) {
                    has_op = true;
                }

                group.push($tokens[i + diff].clone());

                diff += 1;
            }

            if diff > 0 {
                diff -= 1;
            }

            if has_op {
                let node = tokens_to_exp(&group);
                stage.push(node);
                i += diff;
            } else {
                stage.push($tokens[i].clone());
            }

            i += 1;
        }

        stage
    }};
}

pub fn parse_to_node(mut tokens: Vec<ExpressionToken>) -> ExpressionNode {
    {
        let mut stage: Vec<ExpressionToken> = vec![];

        let mut i = 0;
        while i < tokens.len() {
            if let ExpressionToken::LParen = tokens[i] {
                let mut temp_tokens: Vec<ExpressionToken> = vec![];
                i += 1;

                while !matches!(tokens[i], ExpressionToken::RParen) {
                    temp_tokens.push(tokens[i].clone());

                    i += 1;
                }

                let node = parse_to_node(temp_tokens);
                stage.push(ExpressionToken::Node(node));
            } else {
                stage.push(tokens[i].clone());
            }

            i += 1;
        }

        tokens.clone_from(&stage);
    }

    let stage = create_groups!(
        tokens,
        ExpressionToken::Operator(Operator::Exp),
        ExpressionToken::Operator(Operator::Mul)
            | ExpressionToken::Operator(Operator::Div)
            | ExpressionToken::Operator(Operator::Add)
            | ExpressionToken::Operator(Operator::Sub),
    );

    let stage = create_groups!(
        stage,
        ExpressionToken::Operator(Operator::Mul)
            | ExpressionToken::Operator(Operator::Div)
            | ExpressionToken::Node(_),
        ExpressionToken::Operator(Operator::Add) | ExpressionToken::Operator(Operator::Sub),
    );

    let res = tokens_to_exp(&stage);

    let res = match res {
        ExpressionToken::Node(n) => n,
        _ => unreachable!(),
    };

    res
}

fn build_node(
    op: &ExpressionToken,
    pre_token: ExpressionNode,
    post_token: ExpressionNode,
) -> ExpressionNode {
    match op {
        ExpressionToken::Operator(Operator::Add) => {
            ExpressionNode::Add(Box::new(pre_token), Box::new(post_token))
        }
        ExpressionToken::Operator(Operator::Sub) => {
            ExpressionNode::Sub(Box::new(pre_token), Box::new(post_token))
        }
        ExpressionToken::Operator(Operator::Mul) => {
            ExpressionNode::Mul(Box::new(pre_token), Box::new(post_token))
        }
        ExpressionToken::Operator(Operator::Div) => {
            ExpressionNode::Div(Box::new(pre_token), Box::new(post_token))
        }
        ExpressionToken::Operator(Operator::Exp) => {
            ExpressionNode::Exp(Box::new(pre_token), Box::new(post_token))
        }
        _ => unreachable!(),
    }
}

fn tokens_to_exp(tokens: &Vec<ExpressionToken>) -> ExpressionToken {
    if tokens.len() == 1 {
        return tokens[0].clone();
    }

    let mut res: Option<ExpressionNode> = None;

    let mut i = 1;
    while i < tokens.len() {
        if matches!(tokens[i], ExpressionToken::Operator(_)) {
            let pre_token = if let Some(current) = res.clone() {
                current
            } else {
                token_to_node(&tokens[i - 1])
            };
            let post_token = token_to_node(&tokens[i + 1]);

            let node = build_node(&tokens[i], pre_token, post_token);

            res = Some(node);

            i += 1;
        }

        i += 1;
    }

    ExpressionToken::Node(res.expect(&format!("Invalid tokens {:?}", tokens)))
}

fn token_to_node(token: &ExpressionToken) -> ExpressionNode {
    match token {
        ExpressionToken::Number(n) => ExpressionNode::Number(*n),
        ExpressionToken::Node(n) => n.clone(),
        ExpressionToken::Variable(v) => ExpressionNode::Variable(*v),
        _ => panic!("Unexpected token {:?}", token),
    }
}
