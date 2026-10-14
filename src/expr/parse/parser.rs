use crate::expr::{ast::Expr, parse::lexer::Token};

pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
}

pub struct ParseError(String);

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, index: 0 }
    }

    pub fn parse_expr() -> Result<Expr, ParseError> {
        Err(ParseError("a".to_string()))
    }
}
