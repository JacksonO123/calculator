use std::env;
use std::fs;
use std::io;

use crate::math::Derivable;
use crate::parser::parse;
use crate::tokenizer::tokenize;

mod expression;
mod math;
mod parser;
mod tokenizer;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let input_filename = args.get(1).unwrap();

        let input: String = fs::read_to_string("src/input/".trim().to_owned() + input_filename)?;
        println!("{}", input);

        let tokens = tokenize(input);

        let expression = parse(tokens);
        println!("{}", expression);

        let expression = expression.derive();
        println!("d/dx => {}", expression);
    } else {
        repl()?;
    }

    Ok(())
}

fn repl() -> io::Result<()> {
    loop {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf)?;

        if buf == "Exit" {
            break;
        }

        let tokens = tokenize(buf);
        let expression = parse(tokens);
        let derivative = expression.derive();
        println!("{}", derivative);
    }

    Ok(())
}
