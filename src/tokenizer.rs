use std::collections::HashSet;

use crate::expression::{ExpressionToken, Operator};

pub fn tokenize(input: String) -> Vec<ExpressionToken> {
    let mut tokens: Vec<ExpressionToken> = vec![];
    let mut num_stack: Vec<char> = vec![];
    let taken_vars: HashSet<char> = HashSet::new();

    for c in input.chars() {
        if c.is_whitespace() {
            continue;
        }

        if is_number_related(c) {
            num_stack.push(c);
        } else if matches!(c, '(' | ')' | '+' | '-' | '*' | '/' | '^') {
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
                '^' => ExpressionToken::Operator(Operator::Exp),
                _ => unreachable!(),
            };

            tokens.push(token);
        } else {
            if !taken_vars.contains(&c) {
                tokens.push(ExpressionToken::Variable(c))
            }
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
