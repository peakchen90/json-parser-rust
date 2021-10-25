use std::ops::Deref;
use crate::node::{Node, NodeChild};

/// 遍历节点
pub fn walk(node: &Node, cb: fn(&Node)) {
    match node.children.deref() {
        NodeChild::KV { key, value } => {
            cb(node);
            walk(key, cb);
            walk(value, cb);
        }
        NodeChild::List(list) => {
            cb(node);
            for n in list.iter() {
                walk(n, cb);
            }
        }
        NodeChild::Value(_) => {
            cb(node);
        }
        NodeChild::Null => {
            println!("Unexpected Error");
        }
    }
}