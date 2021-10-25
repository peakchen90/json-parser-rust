use crate::parser::Parser;
use crate::node::*;
use crate::token::*;

impl Parser {
    pub fn parse_object_expression(&mut self) -> Node {
        let mut has_tail_comma = false;
        let start_pos = self.current_token.start;
        let mut properties: Vec<Node> = Vec::new();

        self.expect(&TokenType::BracesStart);
        self.move_next();

        while self.is_valid_pos() && self.current_token.token_type != TokenType::BracesEnd {
            has_tail_comma = false;
            let property_start = self.current_token.start;

            // key
            self.expect(&TokenType::String);
            let key = Node::create(
                NodeType::StringLiteral,
                NodeChild::Value(self.current_token.value.to_string()),
                self.current_token.start,
                self.current_token.end,
            );

            // :
            self.move_next();
            self.expect(&TokenType::Separator);

            // value
            self.move_next();
            let value = self.parse_node();
            properties.push(
                Node::create(
                    NodeType::ObjectProperty,
                    NodeChild::KV { key, value },
                    property_start,
                    self.last_token.end,
                )
            );

            // end of property
            if self.current_token.token_type == TokenType::Comma {
                has_tail_comma = true;
                self.move_next();
            } else if self.current_token.token_type != TokenType::BracesEnd {
                break;
            }
        }

        if has_tail_comma {
            self.unexpected_token(&self.last_token);
        }

        self.expect(&TokenType::BracesEnd);
        let node = Node::create(
            NodeType::ObjectExpression,
            NodeChild::List(properties),
            start_pos,
            self.current_token.end,
        );
        self.move_next();
        node
    }

    pub fn parse_array_expression(&mut self) -> Node {
        let mut has_tail_comma = false;
        let start_pos = self.current_token.start;
        let mut elements: Vec<Node> = Vec::new();

        self.expect(&TokenType::BracketsStart);
        self.move_next();

        while self.is_valid_pos() && self.current_token.token_type != TokenType::BracketsEnd {
            has_tail_comma = false;
            elements.push(self.parse_node());

            // end of element
            if self.current_token.token_type == TokenType::Comma {
                has_tail_comma = true;
                self.move_next();
            } else if self.current_token.token_type != TokenType::BracketsEnd {
                break;
            }
        }

        if has_tail_comma {
            self.unexpected_token(&self.last_token);
        }
        self.expect(&TokenType::BracketsEnd);

        let node = Node::create(
            NodeType::ArrayExpression,
            NodeChild::List(elements),
            start_pos,
            self.current_token.end,
        );
        self.move_next();
        node
    }
}
