mod node;
mod token;
mod util;
mod parser;
mod expression;
mod walk;
mod stringifier;

pub mod json {
    use crate::stringifier::Stringifier;
    use crate::parser::Parser;

    pub use crate::node::{Node, NodeType, NodeChild};
    pub use crate::walk::walk;
    pub use crate::token::{Token, TokenType};

    /// 序列化 JSON
    pub fn stringify(node: &Node, indent: u32) -> String {
        let mut stringifier = Stringifier::new(indent);
        stringifier.stringify(&node)
    }

    pub fn parse(input: &str) -> Node {
        Parser::parse(input)
    }
}
