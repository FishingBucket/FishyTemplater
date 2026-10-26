use std::collections::HashMap;

use crate::expr::{
    ast::{BinOp, Expr, ExprKind},
    parse::{
        lexer::Lexer,
        parser::{ParseError, Parser},
    },
    ty::{
        infer::{TypeError, infer_expression},
        symbol::SymbolTable,
        types::{Type, TypeTable},
    },
};

#[derive(Debug)]
pub enum Error {
    ParseError(ParseError),
    TypeError(TypeError),
}

pub fn parse(code: &'static str) -> Result<Expr, Error> {
    let s = code.to_string();
    let mut lexer = Lexer::new(&s);
    lexer.scan();
    let tokens = lexer.get_tokens();
    let mut parser = Parser::new(tokens);
    let expr = parser.expr().map_err(|f| Error::ParseError(f))?;
    let types = TypeTable {
        objects: HashMap::with_capacity(0),
    };
    let symbols = SymbolTable {
        symbols: HashMap::with_capacity(0),
    };
    let typed = infer_expression(expr, &types, &symbols).map_err(|f| Error::TypeError(f))?;
    Ok(typed)
}
