use crate::expr::{
    dice::ast::{
        ComparePoint, Dice, DiceSide, ModCriticalSuccessFail, ModExplodePenetrate, ModKeepDrop,
        ModMinMax, ModRerollUnique, ModSuccessFail,
    },
    ty::{
        infer::{TypeError, infer_expression},
        symbol::SymbolTable,
        types::{Type, TypeTable},
    },
};

fn infer_cp(
    cp: ComparePoint,
    types: &TypeTable,
    symbols: &mut SymbolTable,
) -> Result<ComparePoint, TypeError> {
    let expr = infer_expression(*cp.value, types, symbols)?;
    match &expr.ty {
        Type::Int | Type::Float => Ok(ComparePoint {
            operator: cp.operator,
            value: Box::new(expr),
        }),
        _ => Err(TypeError(
            expr.span,
            format!(
                "expected a number for comparison, got {}",
                expr.ty.describe(types)
            ),
        )),
    }
}

fn infer_min_max(
    minmax: ModMinMax,
    types: &TypeTable,
    symbols: &mut SymbolTable,
) -> Result<ModMinMax, TypeError> {
    let span = minmax.target.span;
    let typed = infer_expression(*minmax.target, types, symbols)?;
    match &typed.ty {
        Type::Int => Ok(ModMinMax {
            target: Box::new(typed),
        }),
        _ => Err(TypeError(
            span,
            format!("expected int, found {}", typed.ty.describe(types)),
        )),
    }
}

fn infer_explode_penetrate(
    explode: ModExplodePenetrate,
    types: &TypeTable,
    symbols: &mut SymbolTable,
) -> Result<ModExplodePenetrate, TypeError> {
    match explode.cp {
        Some(cp) => Ok(ModExplodePenetrate {
            cp: Some(infer_cp(cp, types, symbols)?),
            compound: explode.compound,
        }),
        None => Ok(ModExplodePenetrate {
            cp: None,
            compound: explode.compound,
        }),
    }
}

fn infer_reroll_unique(
    reroll: ModRerollUnique,
    types: &TypeTable,
    symbols: &mut SymbolTable,
) -> Result<ModRerollUnique, TypeError> {
    match reroll.cp {
        Some(cp) => Ok(ModRerollUnique {
            cp: Some(infer_cp(cp, types, symbols)?),
            once: reroll.once,
        }),
        None => Ok(ModRerollUnique {
            cp: None,
            once: reroll.once,
        }),
    }
}

fn infer_keep_drop(
    keep: ModKeepDrop,
    types: &TypeTable,
    symbols: &mut SymbolTable,
) -> Result<ModKeepDrop, TypeError> {
    let expr = infer_expression(*keep.number, types, symbols)?;
    match &expr.ty {
        Type::Int => Ok(ModKeepDrop {
            number: Box::new(expr),
        }),
        _ => Err(TypeError(
            expr.span,
            format!("expected int, found {}", expr.ty.describe(types)),
        )),
    }
}

pub fn infer_dice(
    dice: Dice,
    types: &TypeTable,
    symbols: &mut SymbolTable,
) -> Result<Dice, TypeError> {
    let count = match *dice.count {
        Some(expr) => {
            let span = expr.span;
            let typed = infer_expression(expr, types, symbols)?;
            match &typed.ty {
                Type::Int => Ok(Box::new(Some(typed))),
                _ => Err(TypeError(
                    span,
                    format!("expected int, found {}", typed.ty.describe(types)),
                )),
            }
        }
        None => Ok(Box::new(None)),
    }?;
    let side = match dice.side {
        DiceSide::Percent => Ok(DiceSide::Percent),
        DiceSide::Expr { expr } => {
            let span = expr.span;
            let typed = infer_expression(*expr, types, symbols)?;
            match &typed.ty {
                Type::Int => Ok(DiceSide::Expr {
                    expr: Box::new(typed),
                }),
                _ => Err(TypeError(
                    span,
                    format!("expected int, found {}", typed.ty.describe(types)),
                )),
            }
        }
    }?;
    let min = match dice.min {
        Some(mod_min) => Some(infer_min_max(mod_min, types, symbols)?),
        None => None,
    };
    let max = match dice.max {
        Some(mod_max) => Some(infer_min_max(mod_max, types, symbols)?),
        None => None,
    };
    let explode = match dice.explode {
        Some(explode) => Some(infer_explode_penetrate(explode, types, symbols)?),
        None => None,
    };
    let penetrate = match dice.penetrate {
        Some(penetrate) => Some(infer_explode_penetrate(penetrate, types, symbols)?),
        None => None,
    };
    let reroll = match dice.reroll {
        Some(reroll) => Some(infer_reroll_unique(reroll, types, symbols)?),
        None => None,
    };
    let unique = match dice.unique {
        Some(unique) => Some(infer_reroll_unique(unique, types, symbols)?),
        None => None,
    };
    let keep_high = match dice.keep_high {
        Some(keep_high) => Some(infer_keep_drop(keep_high, types, symbols)?),
        None => None,
    };
    let keep_low = match dice.keep_low {
        Some(keep_low) => Some(infer_keep_drop(keep_low, types, symbols)?),
        None => None,
    };
    let drop_high = match dice.drop_high {
        Some(drop_high) => Some(infer_keep_drop(drop_high, types, symbols)?),
        None => None,
    };
    let drop_low = match dice.drop_low {
        Some(drop_low) => Some(infer_keep_drop(drop_low, types, symbols)?),
        None => None,
    };
    let success = match dice.success {
        Some(success) => Some(ModSuccessFail {
            cp: infer_cp(success.cp, types, symbols)?,
        }),
        None => None,
    };
    let failure = match dice.failure {
        Some(failure) => Some(ModSuccessFail {
            cp: infer_cp(failure.cp, types, symbols)?,
        }),
        None => None,
    };
    let critical_success = match dice.critical_success {
        Some(success) => Some(match success.cp {
            Some(cp) => ModCriticalSuccessFail {
                cp: Some(infer_cp(cp, types, symbols)?),
            },
            None => ModCriticalSuccessFail { cp: None },
        }),
        None => None,
    };
    let critical_failure = match dice.critical_failure {
        Some(failure) => Some(match failure.cp {
            Some(cp) => ModCriticalSuccessFail {
                cp: Some(infer_cp(cp, types, symbols)?),
            },
            None => ModCriticalSuccessFail { cp: None },
        }),
        None => None,
    };
    Ok(Dice {
        count,
        side,
        min,
        max,
        explode,
        penetrate,
        reroll,
        unique,
        keep_high,
        keep_low,
        drop_high,
        drop_low,
        success,
        failure,
        critical_success,
        critical_failure,
        sort: dice.sort,
    })
}
