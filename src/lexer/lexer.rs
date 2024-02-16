use std::collections::HashMap;
use crate::error;
use crate::lexer::token::{LiteralValue, Token};
use crate::lexer::token::TokenType;

pub fn get_keywords() -> HashMap<String, TokenType> {
    [
        ("and".to_string(), TokenType::AND),
        ("class".to_string(), TokenType::CLASS),
        ("aware".to_string(), TokenType::ELSE),
        ("kappa".to_string(), TokenType::FALSE),
        ("forsen".to_string(), TokenType::FOR),
        ("pog".to_string(), TokenType::FUN),
        ("clueless".to_string(), TokenType::IF),
        ("nil".to_string(), TokenType::NIL),
        ("or".to_string(), TokenType::OR),
        ("chatting".to_string(), TokenType::PRINT),
        ("xdd".to_string(), TokenType::RETURN),
        ("super".to_string(), TokenType::SUPER),
        ("this".to_string(), TokenType::THIS),
        ("surely".to_string(), TokenType::TRUE),
        ("lulw".to_string(), TokenType::VAR),
        ("residentsleeper".to_string(), TokenType::WHILE)
    ].iter().cloned().collect()
}

pub struct Lexer {
    source: String,
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    keywords: HashMap<String, TokenType>
}

impl Lexer {
    pub fn new(source: String) -> Lexer {

        Lexer {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            keywords: get_keywords()
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    pub fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token::new(TokenType::EOF, "".to_string(), self.line, None))
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current - 1).unwrap()
    }

    fn add_token(&mut self, kind: TokenType) {
        self.add_token_literal(kind, None);
    }

    fn add_token_literal(&mut self, kind: TokenType, literal: Option<LiteralValue>) {
        let text = self.source[self.start..self.current].to_string();
        self.tokens.push(Token::new(kind, text, self.line, literal));
    }

    fn match_advance(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            error(self.line as i32, "Unterminated string.".to_string());
            return;
        }
        self.advance();
        let value = self.source[self.start + 1..self.current - 1].to_string();
        self.add_token_literal(TokenType::STRING, Some(LiteralValue::StringVal(value)));
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current).unwrap()
    }

    fn number(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }
        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();
            while self.peek().is_digit(10) {
                self.advance();
            }
        }
        let value = self.source[self.start..self.current].parse::<f64>().unwrap();
        self.add_token_literal(TokenType::NUMBER, Some(LiteralValue::FloatVal(value)));
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source.chars().nth(self.current + 1).unwrap()
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() {
            self.advance();
        }
        let text = self.source[self.start..self.current].to_string();
        let kind = self.keywords.get(&text).unwrap_or(&TokenType::IDENTIFIER).clone();
        let kind_val: TokenType = match kind {
            TokenType::AND => TokenType::AND,
            TokenType::CLASS => TokenType::CLASS,
            TokenType::ELSE => TokenType::ELSE,
            TokenType::FALSE => TokenType::FALSE,
            TokenType::FOR => TokenType::FOR,
            TokenType::FUN => TokenType::FUN,
            TokenType::IF => TokenType::IF,
            TokenType::NIL => TokenType::NIL,
            TokenType::OR => TokenType::OR,
            TokenType::PRINT => TokenType::PRINT,
            TokenType::RETURN => TokenType::RETURN,
            TokenType::SUPER => TokenType::SUPER,
            TokenType::THIS => TokenType::THIS,
            TokenType::TRUE => TokenType::TRUE,
            TokenType::VAR => TokenType::VAR,
            TokenType::WHILE => TokenType::WHILE,
            _ => TokenType::IDENTIFIER
        };
        self.add_token(kind_val);
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LEFT_PAREN),
            ')' => self.add_token(TokenType::RIGHT_PAREN),
            '{' => self.add_token(TokenType::LEFT_BRACE),
            '}' => self.add_token(TokenType::RIGHT_BRACE),
            ',' => self.add_token(TokenType::COMMA),
            '.' => self.add_token(TokenType::DOT),
            '-' => self.add_token(TokenType::MINUS),
            '+' => self.add_token(TokenType::PLUS),
            ';' => self.add_token(TokenType::SEMICOLON),
            '*' => self.add_token(TokenType::STAR),
            '!' => {
                if self.match_advance('=') {
                    self.add_token(TokenType::BANG_EQUAL);
                } else {
                    self.add_token(TokenType::BANG);
                }
            },
            '=' => {
                if self.match_advance('=') {
                    self.add_token(TokenType::EQUAL_EQUAL);
                } else {
                    self.add_token(TokenType::EQUAL);
                }
            },
            '<' => {
                if self.match_advance('=') {
                    self.add_token(TokenType::LESS_EQUAL);
                } else {
                    self.add_token(TokenType::LESS);
                }
            },
            '>' => {
                if self.match_advance('=') {
                    self.add_token(TokenType::GREATER_EQUAL);
                } else {
                    self.add_token(TokenType::GREATER);
                }
            },
            '/' => {
                if self.match_advance('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::SLASH);
                }
            },
            ' ' => (),
            '\r' => (),
            '\t' => (),
            '\n' => self.line += 1,
            '"' => self.string(),
            _ => {
                if c.is_digit(10) {
                    self.number();
                } else if c.is_alphabetic() {
                    self.identifier();
                } else {
                    error(self.line as i32, "Unexpected character.".to_string());
                }
            }
        }
    }
}
