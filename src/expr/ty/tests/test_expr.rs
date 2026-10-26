use assert_struct::assert_struct;

use crate::expr::{
    ast::{BinOp, Expr, ExprKind, UnOp},
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

use crate::expr::ty::tests::util::parse;

#[test]
fn test_literals() {
    assert_struct!(
        parse("1"),
        Ok(Expr {
            data: ExprKind::IntLiteral(1),
            ty: Type::Int,
            ..
        })
    );
    assert_struct!(
        parse("1.25"),
        Ok(Expr {
            data: ExprKind::FloatLiteral(1.25),
            ty: Type::Float,
            ..
        })
    );
    assert_struct!(
        parse("true"),
        Ok(Expr {
            data: ExprKind::BoolLiteral(true),
            ty: Type::Bool,
            ..
        })
    );
    assert_struct!(
        parse("\"hello\""),
        Ok(Expr {
            data: ExprKind::StringLiteral("hello"),
            ty: Type::String,
            ..
        })
    );
    assert_struct!(
        parse("null"),
        Ok(Expr {
            data: ExprKind::None,
            ty: Type::None,
            ..
        })
    );
}

#[test]
fn test_binop() {
    assert_struct!(
        parse("1 + 2"),
        Ok(Expr {
            data: ExprKind::BinOp {
                *left: Expr {
                    data: ExprKind::IntLiteral(1),
                    ty: Type::Int,
                    ..
                },
                operator: BinOp::Add,
                *right: Expr {
                    data: ExprKind::IntLiteral(2),
                    ty: Type::Int,
                    ..
                }
            },
            ty: Type::Int,
            ..
        })
    );
    assert_struct!(
        parse("1 + 2.5"),
        Ok(Expr {
            data: ExprKind::BinOp {
                *left: Expr {
                    data: ExprKind::IntLiteral(1),
                    ty: Type::Int,
                    ..
                },
                operator: BinOp::Add,
                *right: Expr {
                    data: ExprKind::FloatLiteral(2.5),
                    ty: Type::Float,
                    ..
                }
            },
            ty: Type::Float,
            ..
        })
    );
    assert_struct!(
        parse("1 && 2"),
        Ok(Expr {
            data: ExprKind::BinOp {
                *left: Expr {
                    data: ExprKind::IntLiteral(1),
                    ty: Type::Int,
                    ..
                },
                operator: BinOp::And,
                *right: Expr {
                    data: ExprKind::IntLiteral(2),
                    ty: Type::Int,
                    ..
                }
            },
            ty: Type::Bool,
            ..
        })
    );
    assert_struct!(
        parse("1 / 2"),
        Ok(Expr {
            data: ExprKind::BinOp {
                *left: Expr {
                    data: ExprKind::IntLiteral(1),
                    ty: Type::Int,
                    ..
                },
                operator: BinOp::Divide,
                *right: Expr {
                    data: ExprKind::IntLiteral(2),
                    ty: Type::Int,
                    ..
                }
            },
            ty: Type::Float,
            ..
        })
    );
}

#[test]
fn test_unop() {
    assert_struct!(
        parse("--1"),
        Ok(Expr {
            data: ExprKind::UnOp {
                operator: UnOp::Negative,
                *value: Expr {
                    data: ExprKind::UnOp {
                        operator: UnOp::Negative,
                        *value: Expr {
                            data: ExprKind::IntLiteral(1),
                            ty: Type::Int,
                            ..
                        }
                    },
                    ty: Type::Int,
                    ..
                }
            },
            ty: Type::Int,
            ..
        })
    );
}
