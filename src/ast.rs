use crate::{span::Span, ty::Type};

mod operators {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Binary {
        Add, Subtract, Times, Divide, Modulus,
        Equals, NotEquals, Less, LessEquals, Greater, GreaterEquals,
        And, Or,
        In,
        BinaryAnd, BinaryOr, BinaryXor
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Unary {
        Not, BinaryNot, Negative, Positive
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Expr {
    pub data: ExprKind,
    pub span: Span,
    pub ty: Type
}

impl Expr {
    pub fn new(data: ExprKind, span: Span, ty: Type) -> Expr {
        Expr {
            data, span, ty
        }
    }

    pub fn unknown(data: ExprKind, span: Span) -> Expr {
        Expr {
            data, span,
            ty: Type::Unknown
        }
    }
}


#[derive(Debug, Clone, PartialEq)]
pub enum ExprKind {
    IntLiteral(i64),
    FloatLiteral(f64),
    StringLiteral(String),
    BoolLiteral(bool),
    Ident(String),
    None(),
    BinOp {
        left: Box<Expr>,
        operator: operators::Binary,
        right: Box<Expr>
    },
    UnOp {
        operator: operators::Unary,
        value: Box<Expr>
    },
    Attribute {
        value: Box<Expr>,
        attribute: String
    },
    Index {
        value: Box<Expr>,
        index: Box<Expr>
    },
    Call {
        value: Box<Expr>,
        arguments: Vec<Expr>
    },
    If {
        condition: Box<Expr>,
        branch_true: Box<Expr>,
        branch_false: Box<Expr>
    }
}