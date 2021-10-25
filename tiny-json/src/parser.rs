use std::rc::Rc;
use crate::node::*;
use crate::token::*;

pub struct Parser {
    pub input: String,
    pub chars: Vec<char>,
    pub length: usize,
    pub pos: usize,
    pub current_token: Rc<Token>,
    pub last_token: Rc<Token>,
}

impl Parser {
    pub fn parse(input: &str) -> Node {
        let chars: Vec<char> = input.chars().collect();
        let length = chars.len();

        let first_token = Rc::new(Token::new(TokenType::StartF));
        let mut parser = Parser {
            input: input.to_string(),
            chars,
            length,
            pos: 0,
            current_token: Rc::clone(&first_token),
            last_token: Rc::clone(&first_token),
        };

        parser.move_next();

        let body = parser.parse_node();
        let root_start = body.start;
        let root_end = body.end;
        let node = Node::create(
            NodeType::Root,
            NodeChild::List(vec![body]),
            root_start,
            root_end,
        );

        if parser.current_token.token_type != TokenType::EndF {
            parser.unexpected_token(&parser.current_token);
        }
        node
    }
}

