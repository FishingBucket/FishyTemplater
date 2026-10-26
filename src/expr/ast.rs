use std::fmt::Display;

use crate::expr::{dice::ast::Dice, ty::types::Type};

use super::span::Span;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BinOp {
    Add,
    Subtract,
    Times,
    Divide,
    Modulus,
    And,
    Or,
    BinaryAnd,
    BinaryOr,
    BinaryXor,
}

impl Display for BinOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinOp::Add => write!(f, "+"),
            BinOp::Subtract => write!(f, "-"),
            BinOp::Times => write!(f, "*"),
            BinOp::Divide => write!(f, "/"),
            BinOp::Modulus => write!(f, "%"),
            BinOp::And => write!(f, "&&"),
            BinOp::Or => write!(f, "||"),
            BinOp::BinaryAnd => write!(f, "&"),
            BinOp::BinaryOr => write!(f, "|"),
            BinOp::BinaryXor => write!(f, "^"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CompOp {
    Equals,
    NotEquals,
    Less,
    LessEquals,
    Greater,
    GreaterEquals,
    In,
}

impl CompOp {
    pub fn is_order(&self) -> bool {
        match self {
            CompOp::Greater | CompOp::GreaterEquals | CompOp::Less | CompOp::LessEquals => true,
            _ => false,
        }
    }

    pub fn is_eq(&self) -> bool {
        match self {
            CompOp::Equals | CompOp::NotEquals => true,
            _ => false,
        }
    }
}

impl Display for CompOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompOp::Equals => write!(f, "=="),
            CompOp::NotEquals => write!(f, "!="),
            CompOp::Less => write!(f, "<"),
            CompOp::LessEquals => write!(f, "<="),
            CompOp::Greater => write!(f, ">"),
            CompOp::GreaterEquals => write!(f, ">="),
            CompOp::In => write!(f, "in"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum UnOp {
    Not,
    BinaryNot,
    Negative,
    Positive,
}

impl Display for UnOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnOp::Not => write!(f, "!"),
            UnOp::BinaryNot => write!(f, "~"),
            UnOp::Negative => write!(f, "-"),
            UnOp::Positive => write!(f, "+"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Expr {
    pub data: ExprKind,
    pub span: Span,
    pub ty: Type,
}

impl Expr {
    pub fn new(span: Span, data: ExprKind) -> Expr {
        Expr {
            data,
            span,
            ty: Type::Unknown,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExprKind {
    IntLiteral(i64),
    FloatLiteral(f64),
    StringLiteral(String),
    BoolLiteral(bool),
    Variable(String),
    None,
    BinOp {
        left: Box<Expr>,
        operator: BinOp,
        right: Box<Expr>,
    },
    Compare {
        operators: Vec<CompOp>,
        values: Vec<Expr>,
    },
    UnOp {
        operator: UnOp,
        value: Box<Expr>,
    },
    Attribute {
        value: Box<Expr>,
        attribute: String,
    },
    Index {
        value: Box<Expr>,
        index: Box<Expr>,
    },
    NameCall {
        name: String,
        arguments: Vec<Expr>,
    },
    Call {
        value: Box<Expr>,
        arguments: Vec<Expr>,
    },
    MethodCall {
        this: Box<Expr>,
        method_name: String,
        arguments: Vec<Expr>,
    },
    If {
        condition: Box<Expr>,
        branch_true: Box<Expr>,
        branch_false: Box<Expr>,
    },
    Arrow {
        predicate: Box<Expr>,
        then: Box<Expr>,
    },
    Dice(Dice),
}
