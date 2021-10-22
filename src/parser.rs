use crate::node::*;
use crate::token::*;
use crate::util::*;

pub struct Parser {
    pub input: String,
    pub chars: Vec<char>,
    pub length: usize,
    pub pos: usize,
    pub current_token: Token,
    pub last_token: Token,
}

impl Parser {
    pub fn parse(input: &String) -> Node {
        let mut chars: Vec<char> = Vec::new();
        for i in input.chars() {
            chars.push(i);
        }
        let length = chars.len();

        let mut parser = Parser {
            input: input.to_string(),
            chars,
            length,
            pos: 0,
            current_token: Token::new(TokenType::StartF),
            last_token: Token::new(TokenType::StartF),
        };

        parser.move_next();

        let body = parser.parse_node();
        let root_start = body.start;
        let root_end = body.end;
        let node = Node::create(
            NodeType::Root,
            NodeChild::Node(body),
            root_start,
            root_end,
        );

        if parser.current_token.token_type != TokenType::EndF {
            parser.unexpected_token(&parser.current_token);
        }
        node
    }
}

