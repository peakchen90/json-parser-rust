use std::ops::Deref;
use crate::node::{Node, NodeChild, NodeType};

pub fn walk(node: &Node) {
    println!("=================== {}", node.to_string());
    match &node.children.deref() {
        NodeChild::KV { key, value } => {
            // cb(node);
            walk(key);
            walk(value);
        },
        NodeChild::List(list) => {
            // cb(node);
            for n in list {
                walk(n);
            }
        },
        NodeChild::Value(_) => {
            // cb(node);
        },
        NodeChild::Null => {
            println!("Unknown node");
        }
    }
}