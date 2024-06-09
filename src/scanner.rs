use std::collections::HashMap;

use crate::{lox, token::{Token, TokenType::{self}}};

#[derive(Default)]
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,

    start: usize,
    current: usize,
    line: usize,

    keywords: HashMap<String, TokenType>
}
impl Scanner {
    pub fn new(source: &str) -> Self {
        let mut keywords = HashMap::new();
        keywords.insert("and", TokenType::And);
        keywords.insert("class", TokenType::Class);
        keywords.insert("else", TokenType::Else);
        keywords.insert("false", TokenType::False);
        keywords.insert("fn", TokenType::Fn);
        keywords.insert("if", TokenType::If);
        keywords.insert("nil", TokenType::Nil);
        keywords.insert("or", TokenType::Or);
        keywords.insert("print", TokenType::Print);
        keywords.insert("return", TokenType::Return);
        keywords.insert("this", TokenType::This);
        keywords.insert("true", TokenType::True);
        keywords.insert("var", TokenType::Var);
        keywords.insert("while", TokenType::While);

        Self { source: source.to_owned(), line: 1, ..Default::default() }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: "".to_owned(),
            line: self.line
        });
        &self.tokens
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                let token_type = if self.next_is('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(token_type)
            },
            '=' => {
                let token_type = if self.next_is('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(token_type);
            },
            '<' => {
                let token_type = if self.next_is('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(token_type);
            },
            '>' => {
                let token_type = if self.next_is('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(token_type);
            },
            '/' => if self.next_is('/') {
                while self.peek() != '\n' && !self.is_at_end() {self.advance();}
            } else {
                self.add_token(TokenType::Slash)
            },
            ' '|'\r'|'\t' => (),
            '\n' => self.line += 1,
            '"' => self.string(), 
            'o' => {
                if self.next_is('r') {
                    self.add_token(TokenType::Or)
                }
            }
            _ => {
                if c.is_ascii_digit() {
                    self.number();
                } else if c.is_alphabetic() {
                    self.identifier()
                } else {
                    lox::error(self.line, "Unexpected Character.");
                }
            }
        }
    }
    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() { self.advance(); }

        let text = &self.source[self.start..self.current];

        let token_type = self.keywords.get(text).cloned().unwrap_or(TokenType::Identifier(text.to_owned()));

        self.add_token(token_type)
    }
    fn number(&mut self) {
        while self.peek().is_ascii_digit() { self.advance(); }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();
            while self.peek().is_ascii_digit() { self.advance(); }
        }

        self.add_token(TokenType::Number(self.source[self.start..self.current].parse().unwrap()))
    }
    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
                self.advance();
            }
        }
        if self.is_at_end() {
            lox::error(self.line, "Unterminated string.");
            return;
        }
        self.advance();
        self.add_token(TokenType::String(self.source[self.start+1..self.current-1].to_owned()));
    }
    fn peek_next(&self) -> char {
        if self.current+1 >= self.source.len() {
            '\0'
        } else {
            self.source.chars().nth(self.current + 1).unwrap()
        }
    }
    fn advance(&mut self) -> char {
        let a = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        a
    }

    fn next_is(&mut self, expected: char) -> bool {
        if self.is_at_end() { return false }
        if self.source.chars().nth(self.current).unwrap() != expected { return false }

        self.current += 1;
        true
    }
    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source.chars().nth(self.current).unwrap()
        }
    }
    fn add_token(&mut self, token_type: TokenType) {
        self.tokens.push(Token {
            token_type,
            lexeme: self.source[self.start..self.current].to_owned(),
            line: self.line
        })
    }
}
