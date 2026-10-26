use assert_struct::assert_struct;

use crate::expr::{
    ast::{CompOp, Expr, ExprKind},
    dice::ast::{
        ComparePoint, Dice, DiceSide, ModCriticalSuccessFail, ModExplodePenetrate, ModMinMax,
        ModRerollUnique, ModSuccessFail,
    },
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
    parser.expr()
}

#[test]
fn test_basic_dice() {
    assert_struct!(
        parse("d6"),
        Ok(Expr {
            data: ExprKind::Dice(Dice {
                *count: None,
                side: DiceSide::Expr {
                    *expr: Expr {
                        data: ExprKind::IntLiteral(6),
                        ..
                    }
                },
                ..
            }),
            ..
        })
    );
    assert_struct!(
        parse("d%"),
        Ok(Expr {
            data: ExprKind::Dice(Dice {
                *count: None,
                side: DiceSide::Percent,
                ..
            }),
            ..
        })
    );
    assert_struct!(
        parse("4d6"),
        Ok(Expr {
            data: ExprKind::Dice(Dice {
                *count: Some(Expr {
                    data: ExprKind::IntLiteral(4),
                    ..
                }),
                side: DiceSide::Expr {
                    *expr: Expr {
                        data: ExprKind::IntLiteral(6),
                        ..
                    }
                },
                ..
            }),
            ..
        })
    );
}

#[test]
fn test_simple_mods() {
    assert_struct!(
        parse("4d6cs"),
        Ok(Expr {
            data: ExprKind::Dice(Dice {
                *count: Some(Expr {
                    data: ExprKind::IntLiteral(4),
                    ..
                }),
                side: DiceSide::Expr {
                    *expr: Expr {
                        data: ExprKind::IntLiteral(6),
                        ..
                    }
                },
                critical_success: Some(
                    ModCriticalSuccessFail {
                        cp: None
                    }
                ),
                ..
            }),
            ..
        })
    );
    assert_struct!(
        parse("4d6min5"),
        Ok(Expr {
            data: ExprKind::Dice(Dice {
                *count: Some(Expr {
                    data: ExprKind::IntLiteral(4),
                    ..
                }),
                side: DiceSide::Expr {
                    *expr: Expr {
                        data: ExprKind::IntLiteral(6),
                        ..
                    }
                },
                min: Some(
                    ModMinMax {
                        *target: Expr {
                            data: ExprKind::IntLiteral(5),
                            ..
                        }
                    }
                ),
                ..
            }),
            ..
        })
    );
}

#[test]
fn test_cp() {
    assert_struct!(
        parse("4d6>3"),
        Ok(Expr {
            data: ExprKind::Dice(Dice {
                *count: Some(Expr {
                    data: ExprKind::IntLiteral(4),
                    ..
                }),
                side: DiceSide::Expr {
                    *expr: Expr {
                        data: ExprKind::IntLiteral(6),
                        ..
                    }
                },
                success: Some(
                    ModSuccessFail {
                        cp: ComparePoint {
                            operator: CompOp::Greater,
                            *value: Expr {
                                data: ExprKind::IntLiteral(3),
                                ..
                            }
                        }
                    }
                ),
                ..
            }),
            ..
        })
    );
    assert_struct!(
        parse("4d6!>=3r"),
        Ok(Expr {
            data: ExprKind::Dice(Dice {
                *count: Some(Expr {
                    data: ExprKind::IntLiteral(4),
                    ..
                }),
                side: DiceSide::Expr {
                    *expr: Expr {
                        data: ExprKind::IntLiteral(6),
                        ..
                    }
                },
                explode: Some(ModExplodePenetrate {
                    cp: Some(ComparePoint {
                        operator: CompOp::GreaterEquals,
                        *value: Expr {
                            data: ExprKind::IntLiteral(3),
                            ..
                        }
                    }),
                    compound: false,
                }),
                reroll: Some(ModRerollUnique {
                    cp: None,
                    once: false,
                }),
                ..
            }),
            ..
        })
    );
}
