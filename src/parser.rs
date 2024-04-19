use crate::expression::{ExpressionToken, Operator};

pub fn parse(tokens: Vec<ExpressionToken>) {
    let mut i = 0;
    while i < tokens.len() {
        let mut index = i;
        let mut group: Vec<ExpressionToken> = vec![];
        let mut has_op = false;

        while !matches!(
            tokens[index],
            ExpressionToken::Operator(Operator::Add) | ExpressionToken::Operator(Operator::Sub)
        ) {
            if matches!(
                tokens[index],
                ExpressionToken::Operator(Operator::Mul) | ExpressionToken::Operator(Operator::Div)
            ) {
                has_op = true;
            }

            group.push(tokens[index]);

            if index == tokens.len() - 1 {
                break;
            }

            index += 1;
        }

        if has_op {
            println!("group: {:?}", group);

            i += index - 3;
        } else {
            println!("here {:?}", tokens[i]);
        }

        i += 1;
    }
}
