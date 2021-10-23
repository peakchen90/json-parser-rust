use std::fs;
use std::ops::Deref;
use crate::parser::Parser;

mod node;
mod token;
mod util;
mod parser;
mod expression;
mod walk;

fn main() {
    println!("\n=============================== OUTPUT ======================================\n");
    let content = fs::read_to_string("test.txt").unwrap();
    let node = Parser::parse(content.as_str());

    walk::walk(&node, &|current| {
        println!("{}", current.to_string());
    })
}
