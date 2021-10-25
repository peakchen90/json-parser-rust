mod node;
mod token;
mod util;
mod parser;
mod expression;
mod walk;
mod stringifier;

use crate::stringifier::Stringifier;
use crate::parser::Parser;

// re-export
pub use crate::node::{Node, NodeType, NodeChild};
pub use crate::walk::walk;
pub use crate::token::{Token, TokenType};

/// 序列化 JSON
///
/// # 参数
/// * `node` - Node 对象
/// * `indent` - 缩进
///
pub fn stringify(node: &Node, indent: u32) -> String {
    let mut stringifier = Stringifier::new(indent);
    stringifier.stringify(&node)
}

/// 解析 JSON
///
/// # 参数
/// * `input` - 解析的 JSON 字符串
///
/// # 示例
/// ```rust
/// let node = tiny_json::parse(r#"{ "a": "hello world"}"#);
/// ```
///
pub fn parse(input: &str) -> Node {
    Parser::parse(input)
}

