use assert_struct::assert_struct;

use crate::expr::{
    ast::{BinOp, CompOp, Expr, ExprKind, UnOp},
    parse::{
        lexer::Lexer,
        parser::{ParseError, Parser},
    },
};

fn parse(code: &'static str) -> Result<Expr, ParseError> {
    let s = code.to_string();
    let mut lexer = Lexer::new(&s);
    lexer.scan();
    let tokens = lexer.get_tokens();
    let mut parser = Parser::new(tokens);
    parser.arrow()
}

#[test]
fn parse_basic_atoms() {
    assert_struct!(
        parse("3.25"),
        Ok(Expr {
            data: ExprKind::FloatLiteral(3.25),
            ..
        })
    );
    assert_struct!(
        parse("3"),
        Ok(Expr {
            data: ExprKind::IntLiteral(3),
            ..
        })
    );
    assert_struct!(
        parse("true"),
        Ok(Expr {
            data: ExprKind::BoolLiteral(true),
            ..
        })
    );
    assert_struct!(
        parse("\"false\""),
        Ok(Expr {
            data: ExprKind::StringLiteral("false"),
            ..
        })
    );
    assert_struct!(
        parse("var"),
        Ok(Expr {
            data: ExprKind::Variable("var"),
            ..
        })
    );
}

#[test]
fn test_ident() {
    assert_struct!(
        parse("dl_link"),
        Ok(Expr {
            data: ExprKind::Variable("dl_link"),
            ..
        })
    )
}

#[test]
fn parse_operations() {
    assert_struct!(
        parse("3 + +8"),
        Ok(Expr {
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
        })
    );
    assert_struct!(
        parse("6 / 2 * (1 + 2)"),
        Ok(Expr {
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
        })
    )
}

#[test]
fn comparisons() {
    assert_struct!(
        parse("1 < 2"),
        Ok(Expr {
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
        })
    );
    assert_struct!(
        parse("1 < 2 < 3 >= 4"),
        Ok(Expr {
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
        })
    );
}

#[test]
fn arrow() {
    assert_struct!(
        parse("true -> false -> \"hello\""),
        Ok(Expr {
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
        })
    )
}

#[test]
fn test_ternary() {
    assert_struct!(
        parse("1 == 1 ? 'true' : 'false'"),
        Ok(Expr {
            data: ExprKind::If {
                *condition: Expr {
                    data: ExprKind::Compare {
                        *left: Expr {
                            data: ExprKind::IntLiteral(1),
                            ..
                        },
                        operators: [CompOp::Equals],
                        values: [
                            Expr {
                                data: ExprKind::IntLiteral(1),
                                ..
                            }
                        ]
                    },
                    ..
                },
                *branch_true: Expr {
                    data: ExprKind::StringLiteral("true"),
                    ..
                },
                *branch_false: Expr {
                    data: ExprKind::StringLiteral("false"),
                    ..
                }
            },
            ..
        })
    );
    assert_struct!(
        parse("true ? true ? true : false : true ? true : false"),
        Ok(Expr {
            data: ExprKind::If {
                *condition: Expr {
                    data: ExprKind::BoolLiteral(true),
                    ..
                },
                *branch_true: Expr {
                    data: ExprKind::If {
                        *condition: Expr {
                            data: ExprKind::BoolLiteral(true),
                            ..
                        },
                        *branch_true: Expr {
                            data: ExprKind::BoolLiteral(true),
                            ..
                        },
                        *branch_false: Expr {
                            data: ExprKind::BoolLiteral(false),
                            ..
                        }
                    },
                    ..
                },
                *branch_false: Expr {
                    data: ExprKind::If {
                        *condition: Expr {
                            data: ExprKind::BoolLiteral(true),
                            ..
                        },
                        *branch_true: Expr {
                            data: ExprKind::BoolLiteral(true),
                            ..
                        },
                        *branch_false: Expr {
                            data: ExprKind::BoolLiteral(false),
                            ..
                        }
                    },
                    ..
                }
            },
            ..
        })
    );
}

#[test]
fn test_call() {
    assert_struct!(
        parse("f(1, 2)"),
        Ok(Expr {
            data: ExprKind::Call {
                *value: Expr {
                    data: ExprKind::Variable("f"),
                    ..
                },
                arguments: [
                    Expr {
                        data: ExprKind::IntLiteral(1),
                        ..
                    },
                    Expr {
                        data: ExprKind::IntLiteral(2),
                        ..
                    }
                ]
            },
            ..
        })
    );
    assert_struct!(
        parse("f(1,)"),
        Ok(Expr {
            data: ExprKind::Call {
                *value: Expr {
                    data: ExprKind::Variable("f"),
                    ..
                },
                arguments: [
                    Expr {
                        data: ExprKind::IntLiteral(1),
                        ..
                    }
                ]
            },
            ..
        })
    );
    assert_struct!(
        parse("f()"),
        Ok(Expr {
            data: ExprKind::Call {
                *value: Expr {
                    data: ExprKind::Variable("f"),
                    ..
                },
                arguments: []
            },
            ..
        })
    );
}

#[test]
fn test_index() {
    assert_struct!(
        parse("'hello'[1]"),
        Ok(Expr {
            data: ExprKind::Index {
                *value: Expr {
                    data: ExprKind::StringLiteral("hello"),
                    ..
                },
                *index: Expr {
                    data: ExprKind::IntLiteral(1),
                    ..
                }
            },
            ..
        })
    )
}

#[test]
fn test_attribute() {
    assert_struct!(
        parse("foo.bar"),
        Ok(Expr {
            data: ExprKind::Attribute {
                *value: Expr {
                    data: ExprKind::Variable("foo"),
                    ..
                },
                attribute: "bar"
            },
            ..
        })
    )
}

#[test]
fn test_trail() {
    assert_struct!(
        parse("'hello'[1].lower()"),
        Ok(Expr {
            data: ExprKind::Call {
                *value: Expr {
                    data: ExprKind::Attribute {
                        *value: Expr {
                            data: ExprKind::Index {
                                *value: Expr {
                                    data: ExprKind::StringLiteral("hello"),
                                    ..
                                },
                                *index: Expr {
                                    data: ExprKind::IntLiteral(1),
                                    ..
                                }
                            },
                            ..
                        },
                        attribute: "lower"
                    },
                    ..
                },
                arguments: []
            },
            ..
        })
    )
}
