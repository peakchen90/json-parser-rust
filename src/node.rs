use std::fmt::Pointer;
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

impl NodeType {
    pub fn to_string(&self) -> String {
        match self {
            NodeType::Root => "Root".to_string(),
            NodeType::ObjectExpression => "ObjectExpression".to_string(),
            NodeType::ObjectProperty => "ObjectProperty".to_string(),
            NodeType::ArrayExpression => "ArrayExpression".to_string(),
            NodeType::StringLiteral => "StringLiteral".to_string(),
            NodeType::NumericLiteral => "NumericLiteral".to_string(),
            NodeType::BooleanLiteral => "BooleanLiteral".to_string(),
            NodeType::NullLiteral => "NullLiteral".to_string(),
        }
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
    pub fn new(node_type: NodeType) -> Node {
        Node {
            node_type,
            start: 0,
            end: 0,
            children: Box::new(NodeChild::Null),
        }
    }

    pub fn create(node_type: NodeType, children: NodeChild, start: usize, end: usize) -> Node {
        Node {
            node_type,
            start,
            end,
            children: Box::new(children),
        }
    }

    pub fn to_string(&self) -> String {
        format!("Node:{} ({}, {})", self.node_type.to_string(), self.start, self.end)
    }
}

impl Parser {
    pub fn parse_node(&mut self) -> Node {
        let current_token = &self.current_token;
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
                if value.eq("null") {
                    let node = Node::create(
                        NodeType::NullLiteral,
                        NodeChild::Value(value),
                        current_token.start,
                        current_token.end,
                    );
                    self.move_next();
                    node
                } else if value.eq("true") || value.eq("false") {
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
