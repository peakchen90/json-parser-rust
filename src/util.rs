use crate::parser::Parser;
use crate::token::*;

pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl Position {
    pub fn new(line: usize, column: usize) -> Position {
        Position {
            line,
            column,
        }
    }

    pub fn from_pos(source_chars: &Vec<char>, pos: usize) -> Position {
        let mut position = Position { line: 1, column: 1 };
        for &c in source_chars {
            if c == '\n' {
                position.line = position.line + 1;
                position.column = 1;
            } else {
                position.column = position.column + 1;
            }
        }
        position
    }
}

pub fn is_number_char(code: usize) -> bool {
    code >= 48 && code <= 57 // 0-9
}

pub fn is_word_char(code: usize) -> bool {
    (code >= 65 && code <= 90) || (code >= 97 && code <= 122) // A-Z or a-z
}

impl Parser {
    pub fn is_valid_pos(&self) -> bool {
        self.pos < self.length
    }

    pub fn get_code_at(&self, pos: usize) -> usize {
        if self.is_valid_pos() {
            self.chars[pos] as usize
        } else { 0 }
    }

    pub fn get_current_code(&self) -> usize {
        self.get_code_at(self.pos)
    }

    pub fn inc_pos(&mut self, num: usize) {
        self.pos = self.pos + num;
    }

    pub fn slice_str(&mut self, start: usize, end: usize) -> String {
        let mut str = String::new();
        for &i in &self.chars[start..end] {
            str.push(i);
        }
        str
    }

    pub fn skip_space(&mut self) {
        while self.is_valid_pos() {
            let code = self.get_current_code();
            if code == 32 || code == 160 { // ` `
                self.inc_pos(1);
            } else if code == 13 || code == 10 || code == 8232 || code == 8233 { // new line
                if code == 13 && self.get_code_at(self.pos + 1) == 10 { // CRLF
                    self.inc_pos(1);
                }
                self.inc_pos(1);
            } else if code > 8 && code < 14 { // 制表符等
                self.inc_pos(1);
            } else {
                break;
            }
        }
    }

    pub fn skip_line_comment(&mut self) {
        let mut code = self.get_current_code();

        if code == 47 && self.get_code_at(self.pos + 1) == 47 { // `//`
            self.inc_pos(1);
            while self.is_valid_pos() {
                self.inc_pos(1);
                code = self.get_current_code();
                if code == 10 { // '\n'
                    self.inc_pos(1);
                    break;
                }
            }
        }
    }

    pub fn skip_block_comment(&mut self) {
        let code = self.get_current_code();
        if code == 47 && self.get_code_at(self.pos + 1) == 42 { // '/*'
            self.inc_pos(1);
            while self.is_valid_pos() {
                self.inc_pos(1);
                self.get_current_code();
                if code == 42 && self.get_code_at(self.pos + 1) == 47 { // '*/'
                    self.inc_pos(2);
                    break;
                }
            }
        }
    }

    pub fn expect(&self, token_type: &TokenType) {
        if &self.current_token.token_type != token_type {
            self.unexpected_token(&self.current_token);
        }
    }

    pub fn unexpected_token(&self, token: &Token) -> ! {
        let mut msg = String::new();
        if token.token_type == TokenType::EndF {
            msg = String::from("Uncaught SyntaxError: Unexpected end of JSON input");
        } else {
            let position = Position::from_pos(&self.chars, token.start);
            msg = format!(
                "Uncaught SyntaxError: Unexpected token {} in JSON at position {} (line {}, column {})",
                token.value,
                token.start,
                position.line,
                position.column,
            );
        }
        panic!(msg);
    }

    pub fn unexpected_pos(&self, pos: usize) -> ! {
        if pos >= self.length {
            panic!("Uncaught SyntaxError: Unexpected end of JSON.")
        }

        let char = &self.chars[pos];
        let position = Position::from_pos(&self.chars, pos);
        let msg = format!(
            "Uncaught SyntaxError: Unexpected token {} in JSON at position {} (line {}, column {})",
            char,
            pos,
            position.line,
            position.column,
        );
        panic!(msg);
    }

    pub fn unexpected_token_type_kind(&self, kind: &TokenTypeKind, pos: usize) -> ! {
        let token_name: &str = match kind {
            TokenTypeKind::String => "number",
            TokenTypeKind::Number => "string"
        };
        let position = Position::from_pos(&self.chars, pos);
        let msg = format!(
            "Uncaught SyntaxError: Unexpected {} in JSON at position {} (line {}, column {})",
            token_name,
            pos,
            position.line,
            position.column,
        );
        panic!(msg);
    }
}

