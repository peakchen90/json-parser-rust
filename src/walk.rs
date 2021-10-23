use std::ops::Deref;
use crate::node::{Node, NodeChild, NodeType};

pub fn walk<T: Fn(&Node)>(node: &Node, cb: &T) {
    match node.children.deref() {
        NodeChild::KV { key, value } => {
            cb(node);
            walk(key, cb);
            walk(value, cb);
        }
        NodeChild::List(list) => {
            cb(node);
            for n in list {
                walk(n, cb);
            }
        }
        NodeChild::Value(x) => {
            cb(node);
        }
        NodeChild::Null => {
            println!("Unexpected Error");
        }
    }
}