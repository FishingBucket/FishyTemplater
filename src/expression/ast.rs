pub mod Operators {
    pub enum Binary {
        Add, Subtract, Times, Divide, Modulus,
        Equals, NotEquals, Less, LessEquals, Greater, GreaterEquals,
        And, Or,
        BinaryAnd, BinaryOr, BinaryXor
    }

    pub enum Unary {
        Not, BinaryNot, Negative, Positive
    }
}

pub enum Expression<'a> {
    IntLiteral(i64),
    FloatLiteral(f64),
    StringLiteral(String),
    BoolLiteral(bool),
    None(),
    BinOp {
        left: Box<&'a Expression<'a>>,
        operator: Operators::Binary,
        right: Box<&'a Expression<'a>>
    },
    UnOp {
        operator: Operators::Unary,
        value: Box<&'a Expression<'a>>
    },

}