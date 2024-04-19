use crate::expression::{ExpressionToken, Operator};

pub fn tokenize(input: String) -> Vec<ExpressionToken> {
    let mut tokens: Vec<ExpressionToken> = vec![];
    let mut num_stack: Vec<char> = vec![];

    for c in input.chars() {
        if c.is_whitespace() {
            continue;
        }

        if is_number_related(c) {
            num_stack.push(c);
        } else {
            if !num_stack.is_empty() {
                let num = stack_to_number(&num_stack);
                tokens.push(ExpressionToken::Number(num));
                num_stack.clear();
            }

            let token = match c {
                '(' => ExpressionToken::LParen,
                ')' => ExpressionToken::RParen,
                '+' => ExpressionToken::Operator(Operator::Add),
                '-' => ExpressionToken::Operator(Operator::Sub),
                '*' => ExpressionToken::Operator(Operator::Mul),
                '/' => ExpressionToken::Operator(Operator::Div),
                _ => panic!("Unexpected token {}", c),
            };

            tokens.push(token);
        }
    }

    if !num_stack.is_empty() {
        let num = stack_to_number(&num_stack);
        tokens.push(ExpressionToken::Number(num));
    }

    tokens
}

fn stack_to_number(stack: &Vec<char>) -> f32 {
    let mut is_neg = false;
    let mut res: f32 = 0.0;
    let mut decimal_found = false;
    let mut decimal_count = 0;

    for c in stack.iter() {
        if decimal_found {
            decimal_count += 1;
        }

        if *c == '-' {
            is_neg = true;
            continue;
        } else if *c == '.' {
            decimal_found = true;
            continue;
        }

        res *= 10.0;
        res += (*c as u32 - 48) as f32;
    }

    if decimal_count > 0 {
        res /= (10 as u32).pow(decimal_count) as f32;
    }

    if is_neg {
        res *= -1.0;
    }

    res
}

fn is_number_related(c: char) -> bool {
    c == '-' || c == '.' || c.is_ascii_digit()
}
