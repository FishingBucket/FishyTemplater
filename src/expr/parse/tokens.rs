use std::fmt::Display;

use crate::expr::span::Span;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    EOF,
    Error(String),

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

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::EOF => write!(f, "EOF"),
            TokenType::Error(_) => write!(f, "Error"),
            TokenType::Plus => write!(f, "'+'"),
            TokenType::Minus => write!(f, "'-'"),
            TokenType::Star => write!(f, "'*'"),
            TokenType::Div => write!(f, "'/'"),
            TokenType::Mod => write!(f, "'%'"),
            TokenType::Que => write!(f, "'?'"),
            TokenType::Colon => write!(f, "':'"),
            TokenType::Eqeq => write!(f, "'=='"),
            TokenType::Lt => write!(f, "'<'"),
            TokenType::Leq => write!(f, "'<='"),
            TokenType::Gt => write!(f, "'>'"),
            TokenType::Geq => write!(f, "'>='"),
            TokenType::Neq => write!(f, "'!='"),
            TokenType::And => write!(f, "'&&'"),
            TokenType::Or => write!(f, "'||'"),
            TokenType::Not => write!(f, "'!'"),
            TokenType::In => write!(f, "'in'"),
            TokenType::BinAnd => write!(f, "'&'"),
            TokenType::BinOr => write!(f, "'|'"),
            TokenType::BinXor => write!(f, "'^'"),
            TokenType::BinNot => write!(f, "'~'"),
            TokenType::Arrow => write!(f, "'->'"),
            TokenType::Dot => write!(f, "'.'"),
            TokenType::Comma => write!(f, "','"),
            TokenType::LParen => write!(f, "'('"),
            TokenType::RParen => write!(f, "')'"),
            TokenType::LBrack => write!(f, "'['"),
            TokenType::RBrack => write!(f, "']'"),
            TokenType::LBrace => write!(f, "'{{'"),
            TokenType::RBrace => write!(f, "'}}'"),
            TokenType::None => write!(f, "'null'"),
            TokenType::Bool(_) => write!(f, "boolean"),
            TokenType::Int(_) => write!(f, "int"),
            TokenType::Float(_) => write!(f, "float"),
            TokenType::String(_) => write!(f, "string"),
            TokenType::Ident(_) => write!(f, "identifier"),
        }
    }
}

pub struct Token {
    pub tty: TokenType,
    pub span: Span,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Token({}, {})", self.tty, self.span)
    }
}
