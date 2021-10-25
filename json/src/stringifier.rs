use std::ops::Deref;
use crate::node::*;
use crate::util::*;
use crate::walk::walk;

pub struct Stringifier {
    config_indent: u32,
    current: u32,
}

impl Stringifier {
    pub fn new(indent: u32) -> Self {
        Stringifier {
            config_indent: indent,
            current: 0,
        }
    }

    pub fn stringify(&mut self, node: &Node) -> String {
        match node.node_type {
            NodeType::Root => {
                let children = node.get_children_as_list();
                self.stringify(&children[0])
            }
            NodeType::ObjectExpression => {
                let mut str = String::from("{");
                self.push_indent();
                let properties = node.get_children_as_list();
                let length = properties.len() as u32;
                let mut i = 0;
                for p in properties {
                    str.push_str(&self.get_indent_str());
                    str.push_str(&self.stringify(p));
                    if i < length - 1 {
                        str.push(',');
                    }
                    i = i + 1;
                }
                self.pop_indent();
                str.push_str(&self.get_indent_str());
                str.push('}');
                str
            }
            NodeType::ObjectProperty => {
                let (key, value) = node.get_children_as_kv();
                ;
                let key_str = &self.stringify(key);
                let value_str = &self.stringify(value);
                let mut str = key_str;
                format!(
                    "{}{}{}",
                    key_str,
                    if self.config_indent > 0 { ": " } else { ":" },
                    value_str
                )
            }
            NodeType::ArrayExpression => {
                let mut str = String::from("[");
                self.push_indent();
                let elements = node.get_children_as_list();
                ;
                let length = elements.len() as u32;
                let mut i = 0;
                for p in elements {
                    str.push_str(&self.get_indent_str());
                    str.push_str(&self.stringify(p));
                    if i < length - 1 {
                        str.push(',');
                    }
                    i = i + 1;
                }
                self.pop_indent();
                str.push_str(&self.get_indent_str());
                str.push(']');
                str
            }
            NodeType::StringLiteral => format!("\"{}\"", escape_str(node.get_children_as_value(), '"')),
            NodeType::NumericLiteral => node.get_children_as_value().to_string(),
            NodeType::BooleanLiteral => node.get_children_as_value().to_string(),
            NodeType::NullLiteral => node.get_children_as_value().to_string()
        }
    }

    fn push_indent(&mut self) -> u32 {
        self.current += self.config_indent;
        self.current
    }

    fn pop_indent(&mut self) -> u32 {
        self.current = self.current - self.config_indent;
        self.current
    }

    fn indent_str(indent_width: u32, prefix: char) -> String {
        let mut str = String::from(prefix);
        for _ in 0..indent_width {
            str.push(' ')
        }
        str
    }

    fn get_indent_str(&self) -> String {
        if self.config_indent == 0 {
            String::from("")
        } else {
            Self::indent_str(self.current, '\n')
        }
    }
}