use crate::expr::parse::lexer::{Lexer, TokenType};

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
}

#[test]
fn divide_or_comment() {
    assert_eq!(
        as_tt_list("/ /"),
        vec![TokenType::Div, TokenType::Div, TokenType::EOF]
    );
    assert_eq!(
        as_tt_list("// hello there"),
        vec![
            TokenType::Comment("hello there".to_string()),
            TokenType::EOF
        ]
    )
}
