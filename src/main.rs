use std::env;
use std::fs;
use std::io;

use crate::parser::parse;
use crate::tokenizer::tokenize;

mod expression;
mod math;
mod parser;
mod tokenizer;

fn main() -> io::Result<()> {
    let thing: Vec<String> = env::args().collect();

    let input_filename = thing.get(1).expect("Expected filename for input");

    let input: String = fs::read_to_string("src/input/".to_owned() + input_filename)?;
    println!("{}", input);

    let tokens = tokenize(input);

    let equation = parse(tokens);
    println!("{:#?}", equation);

    Ok(())
}
