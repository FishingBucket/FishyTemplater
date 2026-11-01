use crate::expr::{
    ast::{BinOp, CompOp, Expr, ExprKind, UnOp},
    span::Span,
    ty::{
        infer_dice::infer_dice,
        symbol::{SymbolID, SymbolTable, SymbolType},
        types::{Function, ParamType, Type, TypeTable},
    },
};

#[derive(Debug, Clone, PartialEq)]
pub struct TypeError(pub Span, pub String);

fn infer_func_call(
    func: &Function,
    arguments: &Vec<Expr>,
    types: &TypeTable,
) -> Result<Type, String> {
    let argument_types = arguments
        .iter()
        .map(|v| v.ty.clone())
        .collect::<Vec<Type>>();
    let mut parameter_index = 0;
    for i in 0..argument_types.len() {
        if parameter_index >= func.params.len() {
            return Err(format!(
                "expected {} arguments, got {}",
                func.params.len(),
                argument_types.len()
            ));
        }
        let argument = &argument_types[i];
        let parameter = &func.params[parameter_index];
        match &parameter {
            ParamType::One(inner) | ParamType::Var(inner) => {
                if !argument.subtypes(inner.clone(), types) {
                    return Err(format!(
                        "{} is not assignable to {}",
                        argument.describe(types),
                        inner.describe(types)
                    ));
                }
            }
        }
        if let ParamType::One(_) = parameter {
            parameter_index += 1;
        }
    }
    if parameter_index < func.params.len() - 1 {
        return Err(format!(
            "expected {} arguments, got {}",
            func.params.len(),
            argument_types.len()
        ));
    }
    Ok(*func.ret.clone())
}

fn infer_name_call_type(
    name: String,
    arguments: &Vec<Expr>,
    types: &TypeTable,
    symbols: &mut SymbolTable,
) -> Result<Type, String> {
    if let Some(variable) = symbols.get_type(SymbolID(name.clone())).clone() {
        match variable {
            SymbolType::SingleType(Type::Function { name: _, func }) => {
                infer_func_call(func, &arguments, types)
            }
            SymbolType::Overloaded(items) => {
                for overload in items {
                    match overload {
                        Type::Function { func, .. } => {
                            if let Ok(result_type) = infer_func_call(func, &arguments, types) {
                                return Ok(result_type);
                            }
                        }
                        _ => {
                            return Err(format!("{} is not a function", name));
                        }
                    }
                }
                Err(format!("no overloads are found for function {}", name))
            }
            _ => Err(format!("{} is not a function", name)),
        }
    } else {
        Err(format!("variable {} is undefined", name))
    }
}

pub fn infer_expression(
    expr: Expr,
    types: &TypeTable,
    symbols: &mut SymbolTable,
) -> Result<Expr, TypeError> {
    let mut expr = expr;
    match expr.data {
        ExprKind::IntLiteral(_) => {
            expr.ty = Type::Int;
            Ok(expr)
        }
        ExprKind::FloatLiteral(_) => {
            expr.ty = Type::Float;
            Ok(expr)
        }
        ExprKind::StringLiteral(_) => {
            expr.ty = Type::String;
            Ok(expr)
        }
        ExprKind::BoolLiteral(_) => {
            expr.ty = Type::Bool;
            Ok(expr)
        }
        ExprKind::Variable(name) => {
            if let Some(t) = symbols.get_type(SymbolID(name.clone())) {
                match t {
                    SymbolType::SingleType(actual_type) => Ok(Expr {
                        data: ExprKind::Variable(name),
                        span: expr.span,
                        ty: actual_type.clone(),
                    }),
                    SymbolType::Overloaded(_) => Err(TypeError(
                        expr.span,
                        "cannot use an overloaded function as a variable".to_string(),
                    )),
                }
            } else {
                Err(TypeError(
                    expr.span,
                    format!("variable '{}' is undefined", name),
                ))
            }
        }
        ExprKind::None => {
            expr.ty = Type::None;
            Ok(expr)
        }
        ExprKind::BinOp {
            left,
            operator,
            right,
        } => {
            let left_inferred = infer_expression(*left, types, symbols)?;
            let right_inferred = infer_expression(*right, types, symbols)?;
            match Type::lub(&left_inferred.ty, &right_inferred.ty, types) {
                Ok(supertype) => match (&operator, &left_inferred.ty, &right_inferred.ty) {
                    (BinOp::Add, Type::String, _) | (BinOp::Add, _, Type::String) => Ok(Expr {
                        span: expr.span,
                        ty: Type::String,
                        data: ExprKind::BinOp {
                            left: Box::new(left_inferred),
                            operator,
                            right: Box::new(right_inferred),
                        },
                    }),
                    (
                        BinOp::Add
                        | BinOp::Subtract
                        | BinOp::Times
                        | BinOp::Modulus
                        | BinOp::Divide,
                        left_type,
                        right_type,
                    ) if left_type.subtypes(Type::Float, types)
                        && right_type.subtypes(Type::Float, types) =>
                    {
                        Ok(Expr {
                            span: expr.span,
                            ty: if operator == BinOp::Divide {
                                Type::Float
                            } else {
                                supertype
                            },
                            data: ExprKind::BinOp {
                                left: Box::new(left_inferred),
                                operator,
                                right: Box::new(right_inferred),
                            },
                        })
                    }
                    (
                        BinOp::BinaryAnd | BinOp::BinaryOr | BinOp::BinaryXor,
                        left_type,
                        right_type,
                    ) if left_type.subtypes(Type::Int, types)
                        && right_type.subtypes(Type::Int, types) =>
                    {
                        Ok(Expr {
                            span: expr.span,
                            ty: Type::Int,
                            data: ExprKind::BinOp {
                                left: Box::new(left_inferred),
                                operator,
                                right: Box::new(right_inferred),
                            },
                        })
                    }
                    (BinOp::And | BinOp::Or, left_type, right_type)
                        if left_type.is_bool() && right_type.is_bool() =>
                    {
                        Ok(Expr {
                            span: expr.span,
                            ty: Type::Bool,
                            data: ExprKind::BinOp {
                                left: Box::new(left_inferred),
                                operator,
                                right: Box::new(right_inferred),
                            },
                        })
                    }
                    (op, a, b) => Err(TypeError(
                        expr.span,
                        format!(
                            "cannot evaluate {} {} {}",
                            a.describe(types),
                            op,
                            b.describe(types)
                        ),
                    )),
                },
                Err(message) => Err(TypeError(expr.span, message)),
            }
        }
        ExprKind::Compare { operators, values } => {
            let mut typed_values = Vec::<Expr>::with_capacity(values.len());
            for value in values {
                typed_values.push(infer_expression(value, types, symbols)?);
            }
            for i in 0..(typed_values.len() - 1) {
                let pair = (&typed_values[i], &typed_values[i + 1]);
                let comp = operators[i];
                match (comp, &pair.0.ty, &pair.1.ty) {
                    (comp, a, b)
                        if comp.is_order()
                            && ((a.subtypes(Type::Float, types)
                                && b.subtypes(Type::Float, types))
                                || (a.subtypes(Type::String, types)
                                    && b.subtypes(Type::String, types))) =>
                    {
                        ()
                    }
                    (CompOp::Equals | CompOp::NotEquals, _, _) => (),
                    (CompOp::In, scalar, Type::List(list_inner))
                        if scalar.subtypes(*list_inner.clone(), types) =>
                    {
                        ()
                    }
                    (CompOp::In, Type::String, Type::String) => (),
                    _ => {
                        return Err(TypeError(
                            pair.0.span + pair.1.span,
                            format!(
                                "cannot compare between {} and {}",
                                pair.0.ty.describe(types),
                                pair.1.ty.describe(types)
                            ),
                        ));
                    }
                }
            }
            Ok(Expr {
                ty: Type::Bool,
                span: expr.span,
                data: ExprKind::Compare {
                    operators,
                    values: typed_values,
                },
            })
        }
        ExprKind::UnOp { operator, value } => {
            let typed_value = infer_expression(*value, types, symbols)?;
            let ty = match (&operator, &typed_value.ty) {
                (UnOp::Not, t) if t.is_bool() => Type::Bool,
                (UnOp::Negative, t) if t.subtypes(Type::Float, types) => t.clone(),
                (UnOp::BinaryNot, Type::Int) => Type::Int,
                (UnOp::Positive, t) if t.subtypes(Type::Float, types) => t.clone(),
                _ => {
                    return Err(TypeError(
                        expr.span,
                        format!(
                            "cannot evaluate {}{}",
                            operator,
                            typed_value.ty.describe(types)
                        ),
                    ));
                }
            };
            Ok(Expr {
                span: expr.span,
                ty,
                data: ExprKind::UnOp {
                    operator,
                    value: Box::new(typed_value),
                },
            })
        }
        ExprKind::Attribute { value, attribute } => {
            let inner_typed = infer_expression(*value, types, symbols)?;
            if let Some(attr_type) = types.get_attr(&inner_typed.ty, attribute.clone()) {
                Ok(Expr {
                    ty: attr_type,
                    span: expr.span,
                    data: ExprKind::Attribute {
                        value: Box::new(inner_typed),
                        attribute,
                    },
                })
            } else {
                Err(TypeError(
                    expr.span,
                    format!(
                        "cannot find {}.{}",
                        inner_typed.ty.describe(types),
                        attribute
                    ),
                ))
            }
        }
        ExprKind::Index { value, index } => {
            let typed_value = infer_expression(*value, types, symbols)?;
            let typed_index = infer_expression(*index, types, symbols)?;
            match (&typed_value.ty, &typed_index.ty) {
                (Type::List(inner), Type::Int) => Ok(Expr {
                    ty: *inner.clone(),
                    span: expr.span,
                    data: ExprKind::Index {
                        value: Box::new(typed_value),
                        index: Box::new(typed_index),
                    },
                }),
                _ => Err(TypeError(
                    expr.span,
                    format!(
                        "cannot index into {} with {}",
                        typed_value.ty.describe(types),
                        typed_index.ty.describe(types)
                    ),
                )),
            }
        }
        ExprKind::NameCall { name, arguments } => {
            let mut typed_arguments = Vec::<Expr>::with_capacity(arguments.len());
            for arg in arguments {
                typed_arguments.push(infer_expression(arg, types, symbols)?);
            }
            let typed_arguments = typed_arguments;
            match infer_name_call_type(name.clone(), &typed_arguments, types, symbols) {
                Ok(ty) => Ok(Expr {
                    span: expr.span,
                    ty,
                    data: ExprKind::NameCall {
                        name,
                        arguments: typed_arguments,
                    },
                }),
                Err(message) => Err(TypeError(expr.span, message)),
            }
        }
        ExprKind::Call { value, arguments } => {
            let mut typed_arguments = Vec::<Expr>::with_capacity(arguments.len());
            for arg in arguments {
                typed_arguments.push(infer_expression(arg, types, symbols)?);
            }
            let typed_arguments = typed_arguments;
            let inferred_call = infer_expression(*value, types, symbols)?;
            match &inferred_call.ty {
                Type::Function { name, func } => {
                    match infer_func_call(func, &typed_arguments, types) {
                        Ok(ty) => Ok(Expr {
                            span: expr.span,
                            ty,
                            data: ExprKind::Call {
                                value: Box::new(inferred_call),
                                arguments: typed_arguments,
                            },
                        }),
                        Err(message) => Err(TypeError(expr.span, message)),
                    }
                }
                _ => Err(TypeError(
                    expr.span,
                    format!("{} is not a function", inferred_call.ty.describe(types)),
                )),
            }
        }
        ExprKind::MethodCall {
            this,
            method_name,
            arguments,
        } => {
            // object.method(param1) <-> method(object, param1)
            let mut typed_arguments = Vec::<Expr>::with_capacity(arguments.len());
            let mut check_arguments = Vec::<Expr>::with_capacity(arguments.len() + 1);
            let typed_this = infer_expression(*this, types, symbols)?;
            check_arguments.push(typed_this.clone());
            for arg in arguments {
                let value = infer_expression(arg, types, symbols)?;
                typed_arguments.push(value.clone());
                check_arguments.push(value.clone());
            }
            let typed_arguments = typed_arguments;
            let check_arguments = check_arguments;
            match infer_name_call_type(method_name.clone(), &check_arguments, types, symbols) {
                Ok(ty) => Ok(Expr {
                    span: expr.span,
                    ty,
                    data: ExprKind::MethodCall {
                        this: Box::new(typed_this),
                        method_name,
                        arguments: typed_arguments,
                    },
                }),
                Err(message) => Err(TypeError(expr.span, message)),
            }
        }
        ExprKind::If {
            condition,
            branch_true,
            branch_false,
        } => {
            let typed_condition = infer_expression(*condition, types, symbols)?;
            if typed_condition.ty.is_bool() {
                let typed_then = infer_expression(*branch_true, types, symbols)?;
                let typed_else = infer_expression(*branch_false, types, symbols)?;
                match Type::lub(&typed_then.ty, &typed_else.ty, types) {
                    Ok(end) => Ok(Expr {
                        span: expr.span,
                        ty: end,
                        data: ExprKind::If {
                            condition: Box::new(typed_condition),
                            branch_true: Box::new(typed_then),
                            branch_false: Box::new(typed_else),
                        },
                    }),
                    Err(message) => Err(TypeError(expr.span, message)),
                }
            } else {
                Err(TypeError(
                    expr.span,
                    format!(
                        "cannot interpret {} as a bool",
                        typed_condition.ty.describe(types)
                    ),
                ))
            }
        }
        ExprKind::Arrow { predicate, then } => {
            let typed_predicate = infer_expression(*predicate, types, symbols)?;
            if typed_predicate.ty.is_bool() {
                let typed_inner = infer_expression(*then, types, symbols)?;
                Ok(Expr {
                    ty: if let Type::Maybe(inner) = &typed_inner.ty {
                        Type::Maybe(inner.clone())
                    } else {
                        Type::Maybe(Box::new(typed_inner.ty.clone()))
                    },
                    data: ExprKind::Arrow {
                        predicate: Box::new(typed_predicate),
                        then: Box::new(typed_inner),
                    },
                    span: expr.span,
                })
            } else {
                Err(TypeError(
                    expr.span,
                    format!(
                        "cannot interpret {} as a bool",
                        typed_predicate.ty.describe(types)
                    ),
                ))
            }
        }
        ExprKind::Walrus { variable, value } => {
            let typed_value = infer_expression(*value, types, symbols)?;
            symbols.insert(
                SymbolID(variable.to_string()),
                SymbolType::SingleType(typed_value.ty.clone()),
            );
            Ok(Expr {
                ty: typed_value.ty.clone(),
                span: expr.span,
                data: ExprKind::Walrus {
                    variable,
                    value: Box::new(typed_value),
                },
            })
        }
        ExprKind::IfWalrus {
            variable,
            condition,
            branch_true,
            branch_false,
        } => {
            let typed_condition = infer_expression(*condition, types, symbols)?;
            if typed_condition.ty.is_bool() {
                if let Type::Maybe(inner) = &typed_condition.ty {
                    symbols.push_layer().insert(
                        SymbolID(variable.to_string()),
                        SymbolType::SingleType(*inner.clone()),
                    );
                }
                let typed_then = infer_expression(*branch_true, types, symbols)?;
                if let Type::Maybe(_) = &typed_condition.ty {
                    symbols.pop_layer();
                    symbols.push_layer().insert(
                        SymbolID(variable.to_string()),
                        SymbolType::SingleType(Type::None),
                    );
                }
                let typed_else = infer_expression(*branch_false, types, symbols)?;
                if let Type::Maybe(_) = &typed_condition.ty {
                    symbols.pop_layer();
                }
                match Type::lub(&typed_then.ty, &typed_else.ty, types) {
                    Ok(end) => Ok(Expr {
                        span: expr.span,
                        ty: end,
                        data: ExprKind::IfWalrus {
                            variable,
                            condition: Box::new(typed_condition),
                            branch_true: Box::new(typed_then),
                            branch_false: Box::new(typed_else),
                        },
                    }),
                    Err(message) => Err(TypeError(expr.span, message)),
                }
            } else {
                Err(TypeError(
                    expr.span,
                    format!(
                        "cannot interpret {} as a bool",
                        typed_condition.ty.describe(types)
                    ),
                ))
            }
        }
        ExprKind::Dice(dice) => Ok(Expr {
            ty: Type::Int,
            span: expr.span,
            data: ExprKind::Dice(infer_dice(dice, types, symbols)?),
        }),
    }
}
