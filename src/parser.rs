use crate::expression::{Expression, ExpressionToken, Operator};

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

pub fn parse(tokens: Vec<ExpressionToken>) -> Expression {
    if tokens.len() == 1 {
        return token_to_node(&tokens[0]);
    }

    let stage = tokens;

    let stage = {
        let mut res: Vec<ExpressionToken> = vec![];

        let mut i = 0;
        while i < stage.len() {
            if let ExpressionToken::Function(name) = &stage[i] {
                let mut num_parens = 1;
                let mut temp_tokens: Vec<ExpressionToken> = vec![];
                i += 2;

                while i < stage.len() && num_parens > 0 {
                    match stage[i] {
                        ExpressionToken::LParen => {
                            num_parens += 1;
                        }
                        ExpressionToken::RParen => {
                            num_parens -= 1;
                        }
                        _ => {}
                    }

                    if num_parens > 0 {
                        temp_tokens.push(stage[i].clone());
                    }

                    i += 1;
                }

                i -= 1;

                let node = parse(temp_tokens);
                res.push(ExpressionToken::Node(Expression::Function(
                    name.clone(),
                    Box::new(node),
                )));
            } else {
                res.push(stage[i].clone());
            }

            i += 1;
        }

        res
    };

    let stage = {
        let mut res: Vec<ExpressionToken> = vec![];

        let mut i = 0;
        while i < stage.len() {
            if let ExpressionToken::LParen = stage[i] {
                let mut temp_tokens: Vec<ExpressionToken> = vec![];
                i += 1;

                while !matches!(stage[i], ExpressionToken::RParen) {
                    temp_tokens.push(stage[i].clone());

                    i += 1;
                }

                let node = parse(temp_tokens);
                res.push(ExpressionToken::Node(node));
            } else {
                res.push(stage[i].clone());
            }

            i += 1;
        }

        res
    };

    let stage = create_groups!(
        stage,
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

    match res {
        ExpressionToken::Node(n) => n,
        _ => panic!("Expected node"),
    }
}

fn build_node(op: &ExpressionToken, pre_token: Expression, post_token: Expression) -> Expression {
    match op {
        ExpressionToken::Operator(Operator::Add) => {
            Expression::Add(Box::new(pre_token), Box::new(post_token))
        }
        ExpressionToken::Operator(Operator::Sub) => {
            Expression::Sub(Box::new(pre_token), Box::new(post_token))
        }
        ExpressionToken::Operator(Operator::Mul) => {
            Expression::Mul(Box::new(pre_token), Box::new(post_token))
        }
        ExpressionToken::Operator(Operator::Div) => {
            Expression::Div(Box::new(pre_token), Box::new(post_token))
        }
        ExpressionToken::Operator(Operator::Exp) => match pre_token {
            Expression::Variable(c, _) => Expression::Variable(c, Box::new(post_token)),
            _ => Expression::Exp(Box::new(pre_token), Box::new(post_token)),
        },
        _ => unreachable!(),
    }
}

fn tokens_to_exp(tokens: &Vec<ExpressionToken>) -> ExpressionToken {
    if tokens.len() == 1 {
        let node = token_to_node(&tokens[0].clone());
        return ExpressionToken::Node(node);
    }

    let mut res: Option<Expression> = None;

    let mut i = 1;
    while i < tokens.len() {
        if i < tokens.len() - 1 && matches!(tokens[i], ExpressionToken::Operator(_)) {
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

    ExpressionToken::Node(res.unwrap_or_else(|| panic!("Invalid tokens {:?}", tokens)))
}

fn token_to_node(token: &ExpressionToken) -> Expression {
    match token {
        ExpressionToken::Number(n) => Expression::Number(*n),
        ExpressionToken::Node(n) => n.clone(),
        ExpressionToken::Variable(v, exp) => Expression::Variable(v.clone(), Box::new(exp.clone())),
        ExpressionToken::Constant(constant) => Expression::Constant(constant.clone()),
        _ => panic!("Unexpected token {:?}", token),
    }
}
