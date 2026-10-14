use crate::expr::span::Span;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    EOF,
    Error(String),
    Comment(String),

    Plus,
    Minus,
    Star,
    Div,
    Mod,

    Que,
    Colon,

    Eqeq,
    Lt,
    Leq,
    Gt,
    Geq,
    Neq,

    And,
    Or,
    Not,
    In,

    BinAnd,
    BinOr,
    BinXor,
    BinNot,

    Arrow,
    Dot,
    Comma,

    LParen,
    RParen,
    LBrack,
    RBrack,
    LBrace,
    RBrace,

    None,
    Bool(bool),

    Int(i64),
    Float(f64),
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

    fn lookahead(&self, n: usize) -> Option<char> {
        self.code.get(self.index + n).map(|v| *v)
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
            if char.is_whitespace() || char == ' ' || char == '\n' {
                self.advance();
            } else {
                return;
            }
        }
    }

    fn add_token(&mut self, tty: TokenType) -> bool {
        self.tokens.push(Token {
            tty,
            span: Span::new(self.pass_start, self.index),
        });
        true
    }

    fn selected(&self) -> String {
        self.code[self.pass_start..self.index].iter().collect()
    }
}

impl Lexer {
    fn scan_next(&mut self) {
        if self.try_lex_divide_or_comment() {
            return;
        }

        if self.try_lex_symbol() {
            return;
        }

        if self.try_lex_number() {
            return;
        }

        if self.try_lex_string() {
            return;
        }

        if self.try_lex_ident() {
            return;
        }

        if let Some(char) = self.peek() {
            self.add_token(TokenType::Error(
                format!("Unknown character {}", char).to_string(),
            ));
        }
    }

    fn try_lex_divide_or_comment(&mut self) -> bool {
        match self.peek() {
            Some('/') => {
                self.advance();
                match self.peek() {
                    Some('/') => {
                        while let Some(char) = self.advance() {
                            if char == '\n' {
                                break;
                            }
                        }
                        self.add_token(TokenType::Comment(self.selected()[2..].trim().to_string()))
                    }
                    _ => self.add_token(TokenType::Div),
                }
            }
            _ => false,
        }
    }

    fn try_lex_symbol(&mut self) -> bool {
        if let Some(char) = self.peek() {
            match char {
                '+' => {
                    self.advance();
                    self.add_token(TokenType::Plus)
                }
                '*' => {
                    self.advance();
                    self.add_token(TokenType::Star)
                }
                '?' => {
                    self.advance();
                    self.add_token(TokenType::Que)
                }
                ':' => {
                    self.advance();
                    self.add_token(TokenType::Colon)
                }
                '.' => {
                    self.advance();
                    self.add_token(TokenType::Dot)
                }
                ',' => {
                    self.advance();
                    self.add_token(TokenType::Comma)
                }
                '(' => {
                    self.advance();
                    self.add_token(TokenType::LParen)
                }
                ')' => {
                    self.advance();
                    self.add_token(TokenType::RParen)
                }
                '[' => {
                    self.advance();
                    self.add_token(TokenType::LBrack)
                }
                ']' => {
                    self.advance();
                    self.add_token(TokenType::RBrack)
                }
                '{' => {
                    self.advance();
                    self.add_token(TokenType::LBrace)
                }
                '}' => {
                    self.advance();
                    self.add_token(TokenType::RBrace)
                }
                '~' => {
                    self.advance();
                    self.add_token(TokenType::BinNot)
                }
                '^' => {
                    self.advance();
                    self.add_token(TokenType::BinXor)
                }
                '!' => match self.lookahead(1) {
                    Some('=') => {
                        self.advance(); // !
                        self.advance(); // =
                        self.add_token(TokenType::Neq)
                    }
                    _ => {
                        self.advance();
                        self.add_token(TokenType::Not)
                    }
                },
                '=' => match self.lookahead(1) {
                    Some('=') => {
                        self.advance(); // =
                        self.advance(); // =
                        self.add_token(TokenType::Eqeq)
                    }
                    _ => false,
                },
                '>' => match self.lookahead(1) {
                    Some('=') => {
                        self.advance(); // >
                        self.advance(); // =
                        self.add_token(TokenType::Geq)
                    }
                    _ => {
                        self.advance();
                        self.add_token(TokenType::Gt)
                    }
                },
                '<' => match self.lookahead(1) {
                    Some('=') => {
                        self.advance(); // <
                        self.advance(); // =
                        self.add_token(TokenType::Leq)
                    }
                    _ => {
                        self.advance();
                        self.add_token(TokenType::Lt)
                    }
                },
                '&' => match self.lookahead(1) {
                    Some('&') => {
                        self.advance(); // &
                        self.advance(); // &
                        self.add_token(TokenType::And)
                    }
                    _ => {
                        self.advance();
                        self.add_token(TokenType::BinAnd)
                    }
                },
                '|' => match self.lookahead(1) {
                    Some('|') => {
                        self.advance(); // |
                        self.advance(); // |
                        self.add_token(TokenType::Or)
                    }
                    _ => {
                        self.advance();
                        self.add_token(TokenType::BinOr)
                    }
                },
                '-' => match self.lookahead(1) {
                    Some('>') => {
                        self.advance(); // -
                        self.advance(); // >
                        self.add_token(TokenType::Arrow)
                    }
                    _ => {
                        self.advance();
                        self.add_token(TokenType::Minus)
                    }
                },
                _ => false,
            }
        } else {
            false
        }
    }

    fn try_lex_number(&mut self) -> bool {
        if !self.peek().map_or(false, |v| '0' <= v && v <= '9') {
            return false;
        }

        let mut has_dot = false;
        while let Some(char) = self.peek() {
            if char == '.' {
                if has_dot {
                    return self.add_token(TokenType::Error("Invalid number".to_string()));
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
                self.add_token(TokenType::Float(f))
            } else {
                self.add_token(TokenType::Error("Invalid number".to_string()))
            }
        } else {
            let res = self.selected().replace("_", "").parse::<i64>();
            if let Ok(i) = res {
                self.add_token(TokenType::Int(i))
            } else {
                self.add_token(TokenType::Error("Invalid number".to_string()))
            }
        }
    }

    fn try_lex_string(&mut self) -> bool {
        if let Some(start_char) = self.peek() {
            let end_quote = match start_char {
                '"' => Some('"'),
                '\'' => Some('\''),
                '\u{00AB}' => Some('\u{00BB}'), // «»
                '\u{2018}' => Some('\u{2019}'), // ‘’
                '\u{201C}' => Some('\u{201D}'), // “”
                '\u{2039}' => Some('\u{203A}'), // ‹›
                _ => None,
            };
            if let Some(end_quo) = end_quote {
                self.lex_string(end_quo);
                return true;
            }
        }
        false
    }

    fn lex_string(&mut self, end_quote: char) {
        self.advance(); // skip the start quote character!
        let mut escaped = false;
        let mut str = String::new();
        while let Some(char) = self.advance() {
            if escaped {
                str.push(match char {
                    'n' => '\n',
                    't' => '\t',
                    'r' => '\r',
                    '0' => '\0',
                    n => n,
                });
                escaped = false;
                continue;
            }
            if char == '\\' {
                escaped = true;
                continue;
            }
            if char == end_quote {
                self.advance();
                self.add_token(TokenType::String(str.to_string()));
                return;
            }
            str.push(char);
        }
        self.add_token(TokenType::Error("Unterminated string".to_string()));
    }

    fn try_lex_ident(&mut self) -> bool {
        if let Some(char) = self.peek() {
            if 'a' <= char && char <= 'z'
                || 'A' <= char && char <= 'Z'
                || char == '_'
                || char == '$'
            {
                self.lex_ident();
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn lex_ident(&mut self) {
        while let Some(char) = self.peek() {
            if 'a' <= char && char <= 'z'
                || 'A' <= char && char <= 'Z'
                || '0' <= char && char <= '9'
                || char == '_'
                || char == '$'
            {
                self.advance();
            } else {
                break;
            }
        }

        let s = self.selected();
        if s == "true" {
            self.add_token(TokenType::Bool(true));
        } else if s == "false" {
            self.add_token(TokenType::Bool(false));
        } else if s == "null" {
            self.add_token(TokenType::None);
        } else if s == "in" {
            self.add_token(TokenType::In);
        } else {
            self.add_token(TokenType::Ident(s));
        }
    }
}
