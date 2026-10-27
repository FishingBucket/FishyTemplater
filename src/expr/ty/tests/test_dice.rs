use assert_struct::assert_struct;

use crate::expr::{
    ast::{BinOp, CompOp, Expr, ExprKind},
    dice::ast::{ComparePoint, Dice, DiceSide, ModExplodePenetrate, ModRerollUnique},
    ty::{tests::util::parse, types::Type},
};

#[test]
fn test_dice() {
    assert_struct!(
        parse("4d6!>=(3 - 2)r"),
        Ok(Expr {
            data: ExprKind::Dice(Dice {
                *count: Some(Expr {
                    data: ExprKind::IntLiteral(4),
                    ty: Type::Int,
                    ..
                }),
                side: DiceSide::Expr {
                    *expr: Expr {
                        data: ExprKind::IntLiteral(6),
                        ty: Type::Int,
                        ..
                    }
                },
                explode: Some(ModExplodePenetrate {
                    cp: Some(ComparePoint {
                        operator: CompOp::GreaterEquals,
                        *value: Expr {
                            data: ExprKind::BinOp {
                                *left: Expr {
                                    data: ExprKind::IntLiteral(3),
                                    ty: Type::Int,
                                    ..
                                },
                                operator: BinOp::Subtract,
                                *right: Expr {
                                    data: ExprKind::IntLiteral(2),
                                    ty: Type::Int,
                                    ..
                                }
                            },
                            ty: Type::Int,
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
            ty: Type::Int,
            ..
        })
    );
}
