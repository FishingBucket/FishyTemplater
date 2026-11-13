use crate::expr::{
    ast::{Expr, ExprKind, UnOp},
    ty::{
        narrow::NarrowKind::{Falsy, Truthy},
        symbol::{SymbolID, SymbolTable, SymbolType},
        types::{Type, TypeTable},
    },
};

#[derive(PartialEq, Eq)]
pub enum NarrowKind {
    Truthy,
    Falsy,
}

pub fn narrow(expr: &Expr, types: &TypeTable, symbols: &mut SymbolTable, kind: NarrowKind) {
    if let ExprKind::UnOp { operator, value } = &expr.data {
        if operator == &UnOp::Not {
            let opposite_kind = match kind {
                Truthy => Falsy,
                Falsy => Truthy,
            };
            return narrow(value, types, symbols, opposite_kind);
        }
    }

    if let ExprKind::Variable(name) | ExprKind::Walrus { variable: name, .. } = &expr.data {
        match (&expr.ty, kind) {
            (Type::Maybe(inner), NarrowKind::Truthy) => {
                symbols.insert(
                    SymbolID(name.to_string()),
                    SymbolType::SingleType(*inner.clone()),
                );
            }
            (Type::Maybe(_), NarrowKind::Falsy) => {
                symbols.insert(
                    SymbolID(name.to_string()),
                    SymbolType::SingleType(Type::None),
                );
            }
            _ => {}
        };
    };
}
