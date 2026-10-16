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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum UnOp {
    Not,
    BinaryNot,
    Negative,
    Positive,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Expr {
    pub data: ExprKind,
    pub span: Span,
}

impl Expr {
    pub fn new(span: Span, data: ExprKind) -> Expr {
        Expr { data, span }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExprKind {
    IntLiteral(i64),
    FloatLiteral(f64),
    StringLiteral(String),
    BoolLiteral(bool),
    Ident(String),
    None,
    BinOp {
        left: Box<Expr>,
        operator: BinOp,
        right: Box<Expr>,
    },
    Compare {
        left: Box<Expr>,
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
    Call {
        value: Box<Expr>,
        arguments: Vec<Expr>,
    },
    If {
        condition: Box<Expr>,
        branch_true: Box<Expr>,
        branch_false: Box<Expr>,
    },
    Arrow {
        predicate: Box<Expr>,
        then: Vec<Expr>,
    },
    Dice {
        count: Box<Expr>,
        side: Box<Expr>,
    },
}
