use assert_struct::assert_struct;

use crate::expr::{
    parse::{
        lexer::Lexer,
        tokens::{IdentType, Token, TokenType},
    },
    span::Span,
};

fn as_tt_list(code: &'static str) -> Vec<TokenType> {
    let s = code.to_string();
    let mut lexer = Lexer::new(&s);
    lexer.scan();
    let tokens = lexer.get_tokens();
    return tokens.iter().map(|t| t.tty.clone()).collect();
}

#[test]
fn numbers() {
    assert_eq!(
        as_tt_list("22187643 12248973   123.25"),
        vec![
            TokenType::Int(22187643),
            TokenType::Int(12248973),
            TokenType::Float(123.25),
            TokenType::EOF
        ]
    );
}

#[test]
fn strings() {
    assert_eq!(
        as_tt_list("'hello\\nthere'"),
        vec![
            TokenType::String("hello\nthere".to_string()),
            TokenType::EOF
        ]
    );
    assert_eq!(
        as_tt_list("'owo''uwu'"),
        vec![
            TokenType::String("owo".to_string()),
            TokenType::String("uwu".to_string()),
            TokenType::EOF,
        ],
    );
}

#[test]
fn divide() {
    assert_eq!(
        as_tt_list("/ /"),
        vec![TokenType::Div, TokenType::Div, TokenType::EOF]
    );
}

#[test]
fn symbols() {
    assert_eq!(
        as_tt_list("+ - * % / ? : == < <= >= > != <> () [] {} && || ! -> & | ~ ^ , ."),
        vec![
            TokenType::Plus,
            TokenType::Minus,
            TokenType::Star,
            TokenType::Mod,
            TokenType::Div,
            TokenType::Que,
            TokenType::Colon,
            TokenType::Eqeq,
            TokenType::Lt,
            TokenType::Leq,
            TokenType::Geq,
            TokenType::Gt,
            TokenType::Neq,
            TokenType::Neq2,
            TokenType::LParen,
            TokenType::RParen,
            TokenType::LBrack,
            TokenType::RBrack,
            TokenType::LBrace,
            TokenType::RBrace,
            TokenType::And,
            TokenType::Or,
            TokenType::Not,
            TokenType::Arrow,
            TokenType::BinAnd,
            TokenType::BinOr,
            TokenType::BinNot,
            TokenType::BinXor,
            TokenType::Comma,
            TokenType::Dot,
            TokenType::EOF
        ]
    )
}

#[test]
fn ident() {
    assert_eq!(
        as_tt_list("true false null in hello $boss o_o"),
        vec![
            TokenType::Bool(true),
            TokenType::Bool(false),
            TokenType::None,
            TokenType::In,
            TokenType::Ident(IdentType::Full("hello".to_string())),
            TokenType::Ident(IdentType::Full("$boss".to_string())),
            TokenType::Ident(IdentType::Full("o_o".to_string())),
            TokenType::EOF
        ]
    )
}

#[test]
fn dice() {
    assert_eq!(
        as_tt_list("4d20k2d2!!<=5cscf"),
        vec![
            TokenType::Int(4),
            TokenType::Ident(IdentType::Split("d".to_string())),
            TokenType::Int(20),
            TokenType::Ident(IdentType::Split("k".to_string())),
            TokenType::Int(2),
            TokenType::Ident(IdentType::Split("d".to_string())),
            TokenType::Int(2),
            TokenType::Not,
            TokenType::Not,
            TokenType::Leq,
            TokenType::Int(5),
            TokenType::Ident(IdentType::Split("cs".to_string())),
            TokenType::Ident(IdentType::Split("cf".to_string())),
            TokenType::EOF
        ]
    );
    assert_struct!(
        {
            let mut lexer = Lexer::new(&"1 dlq".to_string());
            lexer.scan();
            lexer.get_tokens()
        },
        [
            Token {
                tty: TokenType::Int(1),
                span: Span { start: 0, end: 1 }
            },
            Token {
                tty: TokenType::Ident(IdentType::Split("dl")),
                span: Span { start: 2, end: 4 }
            },
            Token {
                tty: TokenType::Ident(IdentType::Split("q")),
                span: Span { start: 4, end: 5 }
            },
            Token {
                tty: TokenType::EOF,
                ..
            }
        ]
    );
    assert_eq!(
        as_tt_list("dldldl"),
        vec![
            TokenType::Ident(IdentType::Split("dl".to_string())),
            TokenType::Ident(IdentType::Split("dl".to_string())),
            TokenType::Ident(IdentType::Split("dl".to_string())),
            TokenType::EOF,
        ]
    )
}
