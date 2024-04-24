use std::collections::{HashMap, HashSet};

use crate::expression::{Expression, ExpressionToken, FunctionName, Operator};

pub fn tokenize(input: String) -> Vec<ExpressionToken> {
    let mut tokens: Vec<ExpressionToken> = vec![];
    let mut num_stack: Vec<char> = vec![];
    let mut taken_vars: HashSet<&str> = HashSet::new();
    let mut taken_fns: HashMap<&str, FunctionName> = HashMap::new();
    let mut var_name = String::new();

    taken_fns.insert("cos", FunctionName::Cos);
    taken_fns.insert("sin", FunctionName::Sin);
    taken_fns.insert("tan", FunctionName::Tan);
    taken_fns.insert("sec", FunctionName::Sec);
    taken_fns.insert("csc", FunctionName::Csc);
    taken_fns.insert("cot", FunctionName::Cot);
    taken_fns.insert("arccos", FunctionName::ArcCos);
    taken_fns.insert("arcsin", FunctionName::ArcSin);
    taken_fns.insert("arctan", FunctionName::ArcTan);
    taken_fns.insert("arcsec", FunctionName::ArcSec);
    taken_fns.insert("arccsc", FunctionName::ArcCsc);
    taken_fns.insert("arccot", FunctionName::ArcCot);
    taken_fns.insert("ln", FunctionName::Ln);
    taken_fns.insert("log", FunctionName::Log);
    taken_fns.insert("abs", FunctionName::Abs);

    taken_vars.insert("e");
    taken_vars.insert("pi");

    for c in input.chars() {
        if c.is_whitespace() {
            if !var_name.is_empty() {
                if let Some(fn_value) = taken_fns.get(var_name.as_str()) {
                    tokens.push(ExpressionToken::Function(fn_value.clone()));
                } else if !taken_vars.contains(var_name.as_str()) {
                    tokens.push(ExpressionToken::Variable(
                        var_name.clone(),
                        Expression::Number(1.0),
                    ));
                }

                var_name.clear();
            }
        } else if c.is_alphabetic() {
            var_name.push(c);
        } else {
            if !var_name.is_empty() {
                if let Some(fn_value) = taken_fns.get(var_name.as_str()) {
                    tokens.push(ExpressionToken::Function(fn_value.clone()));
                } else if !taken_vars.contains(var_name.as_str()) {
                    tokens.push(ExpressionToken::Variable(
                        var_name.clone(),
                        Expression::Number(1.0),
                    ));
                }

                var_name.clear();
            }

            if is_number_related(c) && (c != '-' || num_stack.is_empty()) {
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
            }
        }
    }

    if !num_stack.is_empty() {
        let num = stack_to_number(&num_stack);
        tokens.push(ExpressionToken::Number(num));
    }

    tokens
}

fn stack_to_number(stack: &[char]) -> f32 {
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
        res /= 10_u32.pow(decimal_count) as f32;
    }

    if is_neg {
        res *= -1.0;
    }

    res
}

fn is_number_related(c: char) -> bool {
    c == '-' || c == '.' || c.is_ascii_digit()
}
