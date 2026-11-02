use std::collections::HashMap;

use crate::expr::{
    ast::Expr,
    parse::{
        lexer::Lexer,
        parser::{ParseError, Parser},
    },
    ty::{
        infer::{TypeError, infer_expression},
        symbol::{SymbolID, SymbolTable, SymbolType},
        types::{ConcreteObjectType, Function, ParamType, Type, TypeID, TypeTable},
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
        objects: HashMap::from([(
            TypeID(0),
            ConcreteObjectType {
                fields: HashMap::from([
                    ("foo".to_string(), Type::Int),
                    ("bar".to_string(), Type::List(Box::new(Type::Int))),
                    ("recurse".to_string(), Type::Object(TypeID(0))),
                ]),
            },
        )]),
    };
    let mut symbols = SymbolTable::new(HashMap::from([
        (SymbolID("n".to_string()), SymbolType::SingleType(Type::Int)),
        (
            SymbolID("my_list".to_string()),
            SymbolType::SingleType(Type::List(Box::new(Type::Int))),
        ),
        (
            SymbolID("object".to_string()),
            SymbolType::SingleType(Type::Object(TypeID(0))),
        ),
        (
            SymbolID("max".to_string()),
            SymbolType::Overloaded(vec![
                Type::Function {
                    name: "max".to_string(),
                    func: Function {
                        params: vec![ParamType::Var(Type::Int)],
                        ret: Box::new(Type::Int),
                    },
                },
                Type::Function {
                    name: "max".to_string(),
                    func: Function {
                        params: vec![ParamType::Var(Type::Float)],
                        ret: Box::new(Type::Float),
                    },
                },
            ]),
        ),
        (
            SymbolID("first".to_string()),
            SymbolType::SingleType(Type::Function {
                name: "first".to_string(),
                func: Function {
                    params: vec![ParamType::One(Type::List(Box::new(Type::Int)))],
                    ret: Box::new(Type::Int),
                },
            }),
        ),
        (
            SymbolID("maybe".to_string()),
            SymbolType::SingleType(Type::Maybe(Box::new(Type::Int))),
        ),
    ]));
    let typed = infer_expression(expr, &types, &mut symbols).map_err(|f| Error::TypeError(f))?;
    Ok(typed)
}
