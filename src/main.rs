use std::fs;
use std::ops::Deref;
use crate::parser::Parser;
use crate::stringifier::Stringifier;

mod node;
mod token;
mod util;
mod parser;
mod expression;
mod walk;
mod stringifier;

fn main() {
    println!("\n=============================== OUTPUT ======================================\n");
    let content = fs::read_to_string("test.txt").unwrap();
    let node = Parser::parse(content.as_str());

    // walk::walk(&node, &|current| {
    //     println!("{}", current.to_string());
    // });
    // let result =

    let mut stringifier = Stringifier::new(2);
    let result = stringifier.stringify(&node);
    println!("STRINGIFY:\n{}", result);
}
