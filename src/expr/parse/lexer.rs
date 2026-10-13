use crate::expr::span::Span;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    EOF,
    Error(String),

    Plus,
    Minus,
    Star,
    Div,
    Eqeq,
    Lt,
    Leq,
    Gt,
    Geq,
    Neq,
    LParen,
    RParen,
    LBrack,
    RBrack,
    LBrace,
    RBrace,

    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Ident(String),
}

pub struct Token {
    pub tty: TokenType,
    pub span: Span,
}

pub struct Lexer {
    pass_start: usize,
    index: usize,
    code: Vec<char>,
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(code: &String) -> Lexer {
        Lexer {
            pass_start: 0,
            index: 0,
            code: code.chars().collect(),
            tokens: Vec::with_capacity(10), // should be enough for most uses
        }
    }

    pub fn scan(&mut self) {
        while self.index < self.code.len() {
            self.scan_next();
            self.skip_ws();
            self.pass_start = self.index;
        }
        self.add_token(TokenType::EOF);
    }

    pub fn get_tokens(self) -> Vec<Token> {
        self.tokens
    }
}

impl Lexer {
    fn peek(&self) -> Option<char> {
        self.code.get(self.index).map(|v| *v)
    }

    fn advance(&mut self) -> Option<char> {
        let current = self.peek();
        if current.is_some() {
            self.index += 1;
        }
        current
    }

    fn skip_ws(&mut self) {
        while let Some(char) = self.peek() {
            match char {
                ' ' | '\n' | '\t' | '\r' => {
                    self.advance();
                }
                _ => {
                    break;
                }
            }
        }
    }

    fn add_token(&mut self, tty: TokenType) {
        self.tokens.push(Token {
            tty,
            span: Span::new(self.pass_start, self.index),
        });
    }

    fn selected(&self) -> String {
        self.code[self.pass_start..self.index].iter().collect()
    }
}

impl Lexer {
    fn scan_next(&mut self) {
        if self.start_number() {
            self.lex_number()
        }
    }

    fn start_number(&self) -> bool {
        self.peek().map_or(false, |v| '0' <= v && v <= '9')
    }

    fn lex_number(&mut self) {
        let mut has_dot = false;
        while let Some(char) = self.peek() {
            if char == '.' {
                if has_dot {
                    self.add_token(TokenType::Error("Invalid number".to_string()));
                    return;
                } else {
                    has_dot = true;
                    self.advance();
                }
            } else if '0' <= char && char <= '9' || char == '_' {
                self.advance();
            } else {
                break;
            }
        }
        if has_dot {
            let res = self.selected().replace("_", "").parse::<f64>();
            if let Ok(f) = res {
                self.add_token(TokenType::Float(f));
            } else {
                self.add_token(TokenType::Error("Invalid number".to_string()));
            }
        } else {
            let res = self.selected().replace("_", "").parse::<i64>();
            if let Ok(i) = res {
                self.add_token(TokenType::Int(i));
            } else {
                self.add_token(TokenType::Error("Invalid number".to_string()));
            }
        }
    }
}
