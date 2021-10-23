use std::fs;
use std::ops::Deref;
use crate::parser::Parser;

mod node;
mod token;
mod util;
mod parser;
mod expression;
mod walk;

enum Abc {
    List(Node),
    None,
}

struct Node {
    child: Box<Abc>,
    value: u32,
}

fn test(node: &Node) {
    /*if i > 0 {
        println!("current: {}", i);
        test(i - 1);
    } else {
        println!("END!!");
    }*/

    println!("value: {}", node.value);
    match node.child.deref() {
        Abc::List(child) => {
            test(child);
        }
        Abc::None => {
            println!("END!!");
        }
    }
}

fn main() {
    println!("\n=============================== OUTPUT ======================================\n");
    let node2 = Node {
        child: Box::new(Abc::List(Node {
            child: Box::new(Abc::List(Node {
                child: Box::new(Abc::None),
                value: 3,
            })),
            value: 2,
        })),
        value: 1,
    };


    // test(&node);


    let content = fs::read_to_string("test.txt").unwrap();
    let node = Parser::parse(&content.to_string());

    walk::walk(&node)
}
