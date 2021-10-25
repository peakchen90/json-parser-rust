use std::rc::Rc;
use crate::parser::Parser;
use crate::util::*;

#[derive(Eq, PartialEq)]
pub enum TokenType {
    BracesStart,
    BracesEnd,
    BracketsStart,
    BracketsEnd,
    Separator,
    Comma,
    String,
    Number,
    Word,
    StartF,
    EndF,
    Unknown,
}

#[derive(Eq, PartialEq)]
pub enum TokenTypeKind {
    String,
    Number,
}

pub struct Token {
    pub token_type: TokenType,
    pub value: String,
    pub start: usize,
    pub end: usize,
}

impl Token {
    pub fn new(token_type: TokenType) -> Self {
        Token {
            token_type,
            value: String::from(""),
            start: 0,
            end: 0,
        }
    }

    pub fn create(token_type: TokenType, value: &str, start: usize, end: usize) -> Self {
        Token {
            token_type,
            value: value.to_string(),
            start,
            end,
        }
    }

    pub fn to_string(&self) -> String {
        format!("Token[{}] ({}, {})", self.value, self.start, self.end)
    }
}

impl Parser {
    pub fn move_next(&mut self) {
        let token: Token;
        self.last_token = Rc::clone(&self.current_token);

        let mut start_pos: i32 = -1;
        while self.is_valid_pos() && start_pos != (self.pos as i32) {
            start_pos = self.pos as i32;
            self.skip_space();
            self.skip_line_comment();
            self.skip_block_comment();
        }

        if !self.is_valid_pos() {
            token = Token::create(TokenType::EndF, &"EOF".to_string(), self.length, self.length);
        } else {
            token = self.read_token();
        }
        self.current_token = Rc::new(token);
    }

    pub fn read_token(&mut self) -> Token {
        let code = self.get_current_code();
        if is_number_char(code) || code == 45 { // 0-9 or '-'
            return self.read_number_token();
        } else if is_word_char(code) { // A-Z or a-z
            return self.read_word_token();
        }

        let start_pos = self.pos;
        match code {
            123 => { // '{'
                self.pos += 1;
                Token::create(TokenType::BracesStart, "{", start_pos, self.pos)
            }
            125 => { // '}'
                self.pos += 1;
                Token::create(TokenType::BracesEnd, "}", start_pos, self.pos)
            }
            91 => { // '['
                self.pos += 1;
                Token::create(TokenType::BracketsStart, "[", start_pos, self.pos)
            }
            93 => { // ']'
                self.pos += 1;
                Token::create(TokenType::BracketsEnd, "]", start_pos, self.pos)
            }
            58 => { // ':'
                self.pos += 1;
                Token::create(TokenType::Separator, ":", start_pos, self.pos)
            }
            44 => { // ','
                self.pos += 1;
                Token::create(TokenType::Comma, ",", start_pos, self.pos)
            }
            34 => { // '"'
                self.read_string_token()
            }
            _ => self.unexpected_pos(self.pos)
        }
    }

    pub fn read_string_token(&mut self) -> Token {
        let mut token = Token::new(TokenType::String);
        token.start = self.pos;
        self.pos += 1;

        let mut chunk_start = self.pos;
        let mut code: usize = 0;
        let mut value = String::new();
        let mut is_escape_char = false;

        while self.is_valid_pos() {
            if is_escape_char {
                is_escape_char = false;
                self.pos += 1;
                continue;
            }

            code = self.get_current_code();
            if code == 34 { // '"'
                break;
            }
            if code == 92 { // '\': escape
                is_escape_char = true;
                value.push_str(&self.slice_str(chunk_start, self.pos));
                self.pos += 1;
                chunk_start = self.pos;
            } else {
                self.pos += 1;
            }
        }

        if code != 34 { // '"'
            self.unexpected_token_type_kind(&TokenTypeKind::String, self.pos);
        }

        value.push_str(&self.slice_str(chunk_start, self.pos));
        self.pos += 1;

        token.value = value.to_string();
        token.end = self.pos;
        token
    }

    pub fn read_number_token(&mut self) -> Token {
        let chunk_start = self.pos;
        let mut count = 0;
        let mut allow_dot = false;
        let mut allow_e = false;
        let mut expect_a_number = true;
        let mut code = self.get_current_code();

        if code == 45 { // '-'
            self.pos += 1;
        }

        while self.is_valid_pos() {
            code = self.get_current_code();
            count = count + 1;

            if is_number_char(code) {
                if count == 1 {
                    allow_dot = true;
                    allow_e = true;
                }
                expect_a_number = false;
                self.pos += 1;
            } else if expect_a_number {
                break;
            } else if allow_e && (code == 69 || code == 101) { // 'E' or 'e'
                allow_e = false;
                allow_dot = false;
                expect_a_number = true;
                self.pos += 1;
                if self.is_valid_pos() && (self.get_current_code() == 43 || self.get_current_code() == 45) { // '+' or '-'
                    self.pos += 1;
                }
            } else if allow_dot && code == 46 { // '.'
                allow_dot = false;
                expect_a_number = true;
                self.pos += 1;
            } else {
                break;
            }
        }

        // check
        if expect_a_number {
            self.unexpected_token_type_kind(&TokenTypeKind::Number, self.pos);
        }

        let value = &self.slice_str(chunk_start, self.pos);
        Token::create(TokenType::Number, value, chunk_start, self.pos)
    }

    pub fn read_word_token(&mut self) -> Token {
        let chunk_start = self.pos;
        while self.is_valid_pos() && is_word_char(self.get_code_at(self.pos)) {
            self.pos += 1;
        }
        let value = &self.slice_str(chunk_start, self.pos);
        Token::create(TokenType::Word, value, chunk_start, self.pos)
    }
}
