use std::collections::VecDeque;

use crate::expr::{
    ast::{BinOp, CompOp, Expr, ExprKind, UnOp},
    parse::tokens::{Token, TokenType},
    span::Span,
};

pub struct Parser {
    tokens: VecDeque<Token>,
    last_span: Span,
}

#[derive(Clone, Debug)]
pub struct ParseError(Span, String);

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            last_span: tokens.last().map_or(Span::new(0, 0), |t| t.span),
            tokens: tokens.into(),
        }
    }

    pub fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        let expr = self.parse_arrow()?;
        if let Some(tok) = self.tokens.get(0)
            && tok.tty != TokenType::EOF
        {
            Err(ParseError(
                tok.span,
                format!("Unexpected token {}", tok.tty),
            ))
        } else {
            Ok(expr)
        }
    }
}

impl Parser {
    fn peek(&self) -> Result<&Token, ParseError> {
        let current = self.tokens.get(0);
        if let Some(tok) = current {
            match &tok.tty {
                TokenType::Error(message) => Err(ParseError(tok.span, message.to_string())),
                TokenType::EOF => Err(ParseError(tok.span, "Unexpected end of input".to_string())),
                _ => Ok(tok),
            }
        } else {
            Err(ParseError(
                self.last_span,
                "Unexpected end of input".to_string(),
            ))
        }
    }

    fn advance(&mut self) -> Result<Token, ParseError> {
        self.peek()?; // verify that the token is valid
        Ok(self.tokens.pop_front().unwrap()) // unwrap here is safe
    }

    fn expect(&mut self, tty: TokenType) -> Result<Token, ParseError> {
        let tok = self.advance()?;
        if tok.tty == tty {
            Ok(tok)
        } else {
            Err(ParseError(
                tok.span,
                format!("Expected {}, got {}", tty, tok.tty).to_string(),
            ))
        }
    }

    fn maybe(&mut self, tty: TokenType) -> Option<Token> {
        let tok = self.peek().map_or(None, |v| Some(v));
        if let Some(t) = tok {
            if t.tty == tty {
                self.advance().map_or(None, |v| Some(v))
            } else {
                None
            }
        } else {
            None
        }
    }
}

macro_rules! expect_one_of {
    ($self:tt, $($tty:path),*) => {{
        let tok = $self.advance()?;
        match &tok.tty {
            $($tty)|* => Ok(tok),
            _ => Err(ParseError(
                tok.span,
                format!(
                    "Expected one of {}, got {}",
                    vec![$(format!("{}", $tty)),*].join(", "),
                    tok.tty,
                )
            ))
        }
    }};
}

macro_rules! maybe_one_of {
    ($self:tt, $($tty:path),*) => {{
        let tok = $self.peek();
        if let Ok(t) = tok {
            match &t.tty {
                $($tty)|* => $self.advance().map_or(None, |v| Some(v)),
                _ => None
            }
        } else {
            None
        }
    }};
}

impl Parser {
    fn parse_arrow(&mut self) -> Result<Expr, ParseError> {
        let expr = self.parse_or()?;
        let mut then = Vec::<Expr>::new();
        let mut has = false;
        while let Some(_) = self.maybe(TokenType::Arrow) {
            has = true;
            let right = self.parse_or()?;
            then.push(right);
        }
        if has {
            Ok(Expr::new(
                expr.span + then.last().unwrap().span,
                ExprKind::Arrow {
                    predicate: Box::new(expr),
                    then,
                },
            ))
        } else {
            Ok(expr)
        }
    }

    fn parse_or(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_and()?;
        while let Some(_) = self.maybe(TokenType::Or) {
            let right = self.parse_and()?;
            expr = Expr::new(
                expr.span + right.span,
                ExprKind::BinOp {
                    left: Box::new(expr),
                    operator: BinOp::Or,
                    right: Box::new(right),
                },
            );
        }
        Ok(expr)
    }

    fn parse_and(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_not()?;
        while let Some(_) = self.maybe(TokenType::And) {
            let right = self.parse_not()?;
            expr = Expr::new(
                expr.span + right.span,
                ExprKind::BinOp {
                    left: Box::new(expr),
                    operator: BinOp::And,
                    right: Box::new(right),
                },
            );
        }
        Ok(expr)
    }

    fn parse_not(&mut self) -> Result<Expr, ParseError> {
        if let Some(t) = self.maybe(TokenType::Not) {
            let right = self.parse_not()?;
            Ok(Expr::new(
                t.span + right.span,
                ExprKind::UnOp {
                    operator: UnOp::Not,
                    value: Box::new(right),
                },
            ))
        } else {
            self.parse_compare()
        }
    }

    fn parse_compare(&mut self) -> Result<Expr, ParseError> {
        let expr = self.parse_bit_or()?;
        let mut has = false;
        let mut operators = Vec::<CompOp>::new();
        let mut values = Vec::<Expr>::new();
        while let Some(tok) = maybe_one_of!(
            self,
            TokenType::Eqeq,
            TokenType::Neq,
            TokenType::Lt,
            TokenType::Gt,
            TokenType::Leq,
            TokenType::Geq,
            TokenType::In
        ) {
            has = true;
            let right = self.parse_bit_or()?;
            let op = match tok.tty {
                TokenType::Eqeq => CompOp::Equals,
                TokenType::Neq => CompOp::NotEquals,
                TokenType::Gt => CompOp::Greater,
                TokenType::Geq => CompOp::GreaterEquals,
                TokenType::Lt => CompOp::Less,
                TokenType::Leq => CompOp::LessEquals,
                TokenType::In => CompOp::In,
                _ => unreachable!(),
            };
            operators.push(op);
            values.push(right);
        }
        if has {
            Ok(Expr::new(
                expr.span + values.last().unwrap().span,
                ExprKind::Compare {
                    left: Box::new(expr),
                    operators,
                    values,
                },
            ))
        } else {
            Ok(expr)
        }
    }

    fn parse_bit_or(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_bit_xor()?;
        while let Some(_) = self.maybe(TokenType::BinOr) {
            let right = self.parse_bit_xor()?;
            expr = Expr::new(
                expr.span + right.span,
                ExprKind::BinOp {
                    left: Box::new(expr),
                    operator: BinOp::BinaryOr,
                    right: Box::new(right),
                },
            );
        }
        Ok(expr)
    }

    fn parse_bit_xor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_bit_and()?;
        while let Some(_) = self.maybe(TokenType::BinXor) {
            let right = self.parse_bit_and()?;
            expr = Expr::new(
                expr.span + right.span,
                ExprKind::BinOp {
                    left: Box::new(expr),
                    operator: BinOp::BinaryXor,
                    right: Box::new(right),
                },
            );
        }
        Ok(expr)
    }

    fn parse_bit_and(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_add()?;
        while let Some(_) = self.maybe(TokenType::BinAnd) {
            let right = self.parse_add()?;
            expr = Expr::new(
                expr.span + right.span,
                ExprKind::BinOp {
                    left: Box::new(expr),
                    operator: BinOp::BinaryAnd,
                    right: Box::new(right),
                },
            );
        }
        Ok(expr)
    }

    fn parse_add(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_mult()?;
        while let Some(tok) = maybe_one_of!(self, TokenType::Plus, TokenType::Minus) {
            let operator = match &tok.tty {
                TokenType::Plus => BinOp::Add,
                TokenType::Minus => BinOp::Subtract,
                _ => unreachable!(),
            };
            let right = self.parse_mult()?;
            expr = Expr::new(
                expr.span + right.span,
                ExprKind::BinOp {
                    left: Box::new(expr),
                    operator,
                    right: Box::new(right),
                },
            );
        }
        Ok(expr)
    }

    fn parse_mult(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_un()?;
        while let Some(tok) = maybe_one_of!(self, TokenType::Star, TokenType::Div, TokenType::Mod) {
            let operator = match &tok.tty {
                TokenType::Star => BinOp::Times,
                TokenType::Mod => BinOp::Modulus,
                TokenType::Div => BinOp::Divide,
                _ => unreachable!(),
            };
            let right = self.parse_un()?;
            expr = Expr::new(
                expr.span + right.span,
                ExprKind::BinOp {
                    left: Box::new(expr),
                    operator,
                    right: Box::new(right),
                },
            );
        }
        Ok(expr)
    }

    fn parse_un(&mut self) -> Result<Expr, ParseError> {
        if let Some(t) = maybe_one_of!(self, TokenType::Minus, TokenType::Plus, TokenType::BinNot) {
            let operator = match &t.tty {
                TokenType::Minus => UnOp::Negative,
                TokenType::Plus => UnOp::Positive,
                TokenType::BinNot => UnOp::BinaryNot,
                _ => unreachable!(),
            };
            let right = self.parse_un()?;
            Ok(Expr::new(
                t.span + right.span,
                ExprKind::UnOp {
                    operator,
                    value: Box::new(right),
                },
            ))
        } else {
            self.parse_primary()
        }
    }

    fn parse_primary(&mut self) -> Result<Expr, ParseError> {
        let token = self.advance()?;

        match token.tty {
            TokenType::Bool(b) => Ok(Expr::new(token.span, ExprKind::BoolLiteral(b))),
            TokenType::Int(i) => Ok(Expr::new(token.span, ExprKind::IntLiteral(i))),
            TokenType::Float(f) => Ok(Expr::new(token.span, ExprKind::FloatLiteral(f))),
            TokenType::None => Ok(Expr::new(token.span, ExprKind::None)),
            TokenType::String(s) => Ok(Expr::new(token.span, ExprKind::StringLiteral(s))),
            TokenType::LParen => {
                let inner = self.parse_arrow()?;
                self.expect(TokenType::RParen)?;
                Ok(inner)
            }
            _ => Err(ParseError(token.span, format!("Unexpected {}", token.tty))),
        }
    }
}
