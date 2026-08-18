use crate::expr::{ast::{Expr, ExprKind, operators::Binary}, ty::Type};

use super::env::Environment;

type TypecheckResult = Result<(), String>;

pub fn typecheck(env: &Environment, node: &mut Expr) -> TypecheckResult {
    match node.data {
        ExprKind::IntLiteral(_) => {
            node.ty = Type::Int;
            Ok(())
        },
        ExprKind::FloatLiteral(_) => {
            node.ty = Type::Float;
            Ok(())
        },
        ExprKind::StringLiteral(_) => {
            node.ty = Type::String;
            Ok(())
        },
        ExprKind::BoolLiteral(_) => {
            node.ty = Type::Bool;
            Ok(())
        },
        ExprKind::Ident(name) => {
            let variable_info = env.variables.get(&name).ok_or(format!("Variable {name} not found."))?;
            node.ty = variable_info.ty.clone();
            Ok(())
        },
        ExprKind::None() => {
            node.ty = Type::None;
            Ok(())
        },
        ExprKind::BinOp { left, operator, right } => {
            match operator {
                Binary::Add | Binary::Subtract | Binary::Times | Binary::Divide | Binary::Modulus => {
                    
                },
                Binary::Equals => todo!(),
                Binary::NotEquals => todo!(),
                Binary::Less => todo!(),
                Binary::LessEquals => todo!(),
                Binary::Greater => todo!(),
                Binary::GreaterEquals => todo!(),
                Binary::And => todo!(),
                Binary::Or => todo!(),
                Binary::In => todo!(),
                Binary::BinaryAnd => todo!(),
                Binary::BinaryOr => todo!(),
                Binary::BinaryXor => todo!(),
            }
            Ok(())
        },
        ExprKind::UnOp { operator, value } => todo!(),
        ExprKind::Attribute { value, attribute } => todo!(),
        ExprKind::Index { value, index } => todo!(),
        ExprKind::Call { value, arguments } => todo!(),
        ExprKind::If { condition, branch_true, branch_false } => todo!(),
    }
}

fn unify(to: &Type, from: &Type) -> Result<Type, String> {
    match (to, from) {
        (Type::Unknown, _) => Err("Cannot unify to unknown type".to_string()),
        (_, Type::Unknown) => Err("".to_string()),
        (Type::Any, _) => Ok(Type::Any),
        (Type::Float, Type::Int) => Ok(Type::Float),
        (Type::Int, Type::Bool) => Ok(Type::Int),
        (Type::Array(inner_to), Type::Array(inner_from)) =>
            Ok(Type::Array(Box::new(unify(inner_to, inner_from)?))),
        (a, b) if a == b => Ok(a.clone()),
        _ => Ok(Type::Any)
    }
}