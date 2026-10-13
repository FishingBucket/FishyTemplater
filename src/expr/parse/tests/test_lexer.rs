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
        as_tt_list("12345 123"),
        vec![TokenType::Int(12345), TokenType::Int(123), TokenType::EOF]
    );
}
