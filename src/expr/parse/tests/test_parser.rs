use assert_struct::assert_struct;

use crate::expr::{
    ast::{BinOp, CompOp, Expr, ExprKind, UnOp},
    parse::{lexer::Lexer, parser::Parser},
};

fn parse(code: &'static str) -> Expr {
    let s = code.to_string();
    let mut lexer = Lexer::new(&s);
    lexer.scan();
    let tokens = lexer.get_tokens();
    let mut parser = Parser::new(tokens);
    parser.parse_expr().unwrap()
}

#[test]
fn parse_basic_atoms() {
    assert_struct!(
        parse("3.25"),
        Expr {
            data: ExprKind::FloatLiteral(3.25),
            ..
        }
    );
    assert_struct!(
        parse("3"),
        Expr {
            data: ExprKind::IntLiteral(3),
            ..
        }
    );
    assert_struct!(
        parse("true"),
        Expr {
            data: ExprKind::BoolLiteral(true),
            ..
        }
    );
    assert_struct!(
        parse("\"false\""),
        Expr {
            data: ExprKind::StringLiteral("false"),
            ..
        }
    );
}

#[test]
fn parse_operations() {
    assert_struct!(
        parse("3 + +8"),
        Expr {
            data: ExprKind::BinOp {
                *left: Expr {
                    data: ExprKind::IntLiteral(3),
                    ..
                },
                operator: BinOp::Add,
                *right: Expr {
                    data: ExprKind::UnOp {
                        operator: UnOp::Positive,
                        *value: Expr {
                            data: ExprKind::IntLiteral(8),
                            ..
                        }
                    },
                    ..
                }
            },
            ..
        }
    );
    assert_struct!(
        parse("6 / 2 * (1 + 2)"),
        Expr {
            data: ExprKind::BinOp {
                *left: Expr {
                    data: ExprKind::BinOp {
                        *left: Expr {
                            data: ExprKind::IntLiteral(6),
                            ..
                        },
                        operator: BinOp::Divide,
                        *right: Expr {
                            data: ExprKind::IntLiteral(2),
                            ..
                        }
                    },
                    ..
                },
                operator: BinOp::Times,
                *right: Expr {
                    data: ExprKind::BinOp {
                        *left: Expr {
                            data: ExprKind::IntLiteral(1),
                            ..
                        },
                        operator: BinOp::Add,
                        *right: Expr {
                            data: ExprKind::IntLiteral(2),
                            ..
                        }
                    },
                    ..
                }
            },
            ..
        }
    )
}

#[test]
fn comparisons() {
    assert_struct!(
        parse("1 < 2"),
        Expr {
            data: ExprKind::Compare {
                *left: Expr {
                    data: ExprKind::IntLiteral(1),
                    ..
                },
                operators: [CompOp::Less],
                values: [
                    Expr {
                        data: ExprKind::IntLiteral(2),
                        ..
                    }
                ]
            },
            ..
        }
    );
    assert_struct!(
        parse("1 < 2 < 3 >= 4"),
        Expr {
            data: ExprKind::Compare {
                *left: Expr {
                    data: ExprKind::IntLiteral(1),
                    ..
                },
                operators: [CompOp::Less, CompOp::Less, CompOp::GreaterEquals],
                values: [
                    Expr {
                        data: ExprKind::IntLiteral(2),
                        ..
                    },
                    Expr {
                        data: ExprKind::IntLiteral(3),
                        ..
                    },
                    Expr {
                        data: ExprKind::IntLiteral(4),
                        ..
                    }
                ]
            },
            ..
        }
    );
}

#[test]
fn arrow() {
    assert_struct!(
        parse("true -> false -> \"hello\""),
        Expr {
            data: ExprKind::Arrow {
                *predicate: Expr {
                    data: ExprKind::BoolLiteral(true),
                    ..
                },
                then: [
                    Expr {
                        data: ExprKind::BoolLiteral(false),
                        ..
                    },
                    Expr {
                        data: ExprKind::StringLiteral("hello"),
                        ..
                    }
                ]
            },
            ..
        }
    )
}
