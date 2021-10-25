use std::fmt::{Display, Formatter};
use std::ops::Deref;
use crate::parser::Parser;
use crate::token::TokenType;

pub enum NodeType {
    Root,
    ObjectExpression,
    ObjectProperty,
    ArrayExpression,
    StringLiteral,
    NumericLiteral,
    BooleanLiteral,
    NullLiteral,
}

impl Display for NodeType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            NodeType::Root => String::from("Root"),
            NodeType::ObjectExpression => String::from("ObjectExpression"),
            NodeType::ObjectProperty => String::from("ObjectProperty"),
            NodeType::ArrayExpression => String::from("ArrayExpression"),
            NodeType::StringLiteral => String::from("StringLiteral"),
            NodeType::NumericLiteral => String::from("NumericLiteral"),
            NodeType::BooleanLiteral => String::from("BooleanLiteral"),
            NodeType::NullLiteral => String::from("NullLiteral"),
        })
    }
}

pub enum NodeChild {
    KV { key: Node, value: Node },
    List(Vec<Node>),
    Value(String),
    Null,
}

pub struct Node {
    pub node_type: NodeType,
    pub start: usize,
    pub end: usize,
    pub children: Box<NodeChild>,
}

impl Node {
    pub fn new(node_type: NodeType) -> Self {
        Node {
            node_type,
            start: 0,
            end: 0,
            children: Box::new(NodeChild::Null),
        }
    }

    pub fn create(node_type: NodeType, children: NodeChild, start: usize, end: usize) -> Self {
        Node {
            node_type,
            start,
            end,
            children: Box::new(children),
        }
    }

    pub fn get_children_as_value(&self) -> &String {
        match self.children.deref() {
            NodeChild::Value(x) => x,
            _ => panic!("Invalid visit children of Node")
        }
    }

    pub fn get_children_as_list(&self) -> &Vec<Node> {
        match self.children.deref() {
            NodeChild::List(x) => x,
            _ => panic!("Invalid visit children of Node")
        }
    }

    pub fn get_children_as_kv(&self) -> (&Node, &Node) {
        match self.children.deref() {
            NodeChild::KV { key, value } => (key, value),
            _ => panic!("Invalid visit children of Node")
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Node::{} ({}, {})", self.node_type, self.start, self.end)
    }
}

impl Parser {
    pub fn parse_node(&mut self) -> Node {
        let current_token = self.current_token.deref();
        let start_pos = current_token.start;
        let value = current_token.value.to_string();

        let mut node = match current_token.token_type {
            TokenType::BracesStart => self.parse_object_expression(),
            TokenType::BracketsStart => self.parse_array_expression(),
            TokenType::String => {
                let node = Node::create(
                    NodeType::StringLiteral,
                    NodeChild::Value(value),
                    current_token.start,
                    current_token.end,
                );
                self.move_next();
                node
            }
            TokenType::Number => {
                let node = Node::create(
                    NodeType::NumericLiteral,
                    NodeChild::Value(value),
                    current_token.start,
                    current_token.end,
                );
                self.move_next();
                node
            }
            TokenType::Word => {
                if value == "null" {
                    let node = Node::create(
                        NodeType::NullLiteral,
                        NodeChild::Value(value),
                        current_token.start,
                        current_token.end,
                    );
                    self.move_next();
                    node
                } else if value == "true" || value == "false" {
                    let node = Node::create(
                        NodeType::BooleanLiteral,
                        NodeChild::Value(value),
                        current_token.start,
                        current_token.end,
                    );
                    self.move_next();
                    node
                } else {
                    self.unexpected_token(current_token);
                }
            }
            _ => self.unexpected_token(current_token)
        };

        node.start = start_pos;
        node.end = self.last_token.end;
        node
    }
}
