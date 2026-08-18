use derive_more::Display;
use super::{span::Span, ty::Type};

pub mod operators {
    use derive_more::Display;

    #[derive(Display, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Binary {
        Add, Subtract, Times, Divide, Modulus,
        Equals, NotEquals, Less, LessEquals, Greater, GreaterEquals,
        And, Or,
        In,
        BinaryAnd, BinaryOr, BinaryXor
    }

    #[derive(Display, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Unary {
        Not, BinaryNot, Negative, Positive
    }
}

#[derive(Display, Debug, Clone, PartialEq)]
#[display("Expr({data}, {span}, {ty})")]
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


#[derive(Display, Debug, Clone, PartialEq)]
pub enum ExprKind {
    IntLiteral(i64),
    FloatLiteral(f64),
    StringLiteral(String),
    BoolLiteral(bool),
    Ident(String),
    None(),
    #[display("({left} {operator} {right})")]
    BinOp {
        left: Box<Expr>,
        operator: operators::Binary,
        right: Box<Expr>
    },
    #[display("({operator}{value})")]
    UnOp {
        operator: operators::Unary,
        value: Box<Expr>
    },
    #[display("({value}.{attribute})")]
    Attribute {
        value: Box<Expr>,
        attribute: String
    },
    #[display("({value}[{index}])")]
    Index {
        value: Box<Expr>,
        index: Box<Expr>
    },
    #[display("({value}({arguments:?}))")]
    Call {
        value: Box<Expr>,
        arguments: Vec<Expr>
    },
    #[display("({condition} ? {branch_true} : {branch_false})")]
    If {
        condition: Box<Expr>,
        branch_true: Box<Expr>,
        branch_false: Box<Expr>
    }
}