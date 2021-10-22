use std::fs;
use crate::parser::Parser;

mod node;
mod token;
mod util;
mod parser;
mod expression;
// mod walk;

fn main() {
    println!("\n=============================== OUTPUT ======================================\n");
    let content = fs::read_to_string("test.txt").unwrap();
    let mut parser = Parser::parse(&String::from(content));
}
