use std::collections::VecDeque;

use crate::expr::{
    ast::{BinOp, CompOp, Expr, ExprKind, UnOp},
    dice::ast::DiceSide,
    parse::tokens::{IdentType, Token, TokenType},
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

    pub fn expr(&mut self) -> Result<Expr, ParseError> {
        let expr = self.parse_expr()?;
        self.check_end()?;
        Ok(expr)
    }

    pub fn arrow(&mut self) -> Result<Expr, ParseError> {
        let expr = self.parse_arrow()?;
        self.check_end()?;
        Ok(expr)
    }

    fn check_end(&self) -> Result<(), ParseError> {
        if let Some(tok) = self.tokens.get(0)
            && tok.tty != TokenType::EOF
        {
            Err(ParseError(
                tok.span,
                format!("Unexpected token {}", tok.tty),
            ))
        } else {
            Ok(())
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

    fn expect(&mut self, tty: TokenType, location: &'static str) -> Result<Token, ParseError> {
        let tok = self.advance()?;
        if tok.tty == tty {
            Ok(tok)
        } else {
            Err(ParseError(
                tok.span,
                format!("Expected {} {}, got {}", tty, location, tok.tty).to_string(),
            ))
        }
    }

    fn maybe(&mut self, tty: TokenType) -> Option<Token> {
        let tok = self.peek().ok();
        if let Some(t) = tok {
            if t.tty == tty {
                self.advance().ok()
            } else {
                None
            }
        } else {
            None
        }
    }

    fn maybe_identifier(&mut self) -> Option<(String, Span)> {
        let tok = self.peek().ok()?;
        if let TokenType::Ident(_) = &tok.tty {
            let tok = self.advance().ok()?;
            match tok.tty {
                TokenType::Ident(IdentType::Full(s)) => Some((s.to_string(), tok.span)),
                TokenType::Ident(IdentType::Split(s)) => {
                    let mut full = s.to_string();
                    let mut span = tok.span;
                    loop {
                        let peek = self.peek().ok();
                        if let Some(t) = peek
                            && let TokenType::Ident(IdentType::Split(s)) = &t.tty
                        {
                            full += s;
                            span = span + t.span;
                            self.advance().ok()?;
                        } else {
                            break;
                        }
                    }
                    Some((full, span))
                }
                _ => unreachable!(),
            }
        } else {
            None
        }
    }

    fn maybe_soft_keyword(&mut self, keyword: &'static str) -> Option<Token> {
        let tok = self.peek().ok();
        if let Some(t) = tok {
            match &t.tty {
                TokenType::Ident(IdentType::Full(s) | IdentType::Split(s)) => {
                    if s == keyword {
                        self.advance().ok()
                    } else {
                        None
                    }
                }
                _ => None,
            }
        } else {
            None
        }
    }
}

macro_rules! maybe_one_of {
    ($self:tt, $($tty:path),*) => {{
        let tok = $self.peek();
        if let Ok(t) = tok {
            match &t.tty {
                $($tty)|* => $self.advance().ok(),
                _ => None
            }
        } else {
            None
        }
    }};
}

impl Parser {
    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        self.parse_if()
    }

    fn parse_arrow(&mut self) -> Result<Expr, ParseError> {
        let expr = self.parse_if()?;
        let mut then = Vec::<Expr>::new();
        let mut has = false;
        while let Some(_) = self.maybe(TokenType::Arrow) {
            has = true;
            let right = self.parse_if()?;
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

    fn parse_if(&mut self) -> Result<Expr, ParseError> {
        let expr = self.parse_or()?;
        if let Some(_) = self.maybe(TokenType::Que) {
            let then = self.parse_if()?;
            self.expect(
                TokenType::Colon,
                "after the true branch of a ternary expression",
            )?;
            let otherwise = self.parse_if()?;
            Ok(Expr::new(
                expr.span + otherwise.span,
                ExprKind::If {
                    condition: Box::new(expr),
                    branch_true: Box::new(then),
                    branch_false: Box::new(otherwise),
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
            TokenType::Neq2,
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
                TokenType::Neq2 => CompOp::NotEquals,
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
            self.parse_trail()
        }
    }

    fn parse_trail(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_dice()?;
        while let Some(tok) =
            maybe_one_of!(self, TokenType::LParen, TokenType::LBrack, TokenType::Dot)
        {
            match tok.tty {
                TokenType::LParen => {
                    let mut arguments = Vec::<Expr>::new();
                    loop {
                        if let Ok(tok) = self.peek()
                            && tok.tty == TokenType::RParen
                        {
                            break;
                        }
                        arguments.push(self.parse_expr()?);
                        if self.maybe(TokenType::Comma).is_none() {
                            break;
                        }
                    }
                    let rparen =
                        self.expect(TokenType::RParen, "after the end of an argument list")?;
                    expr = Expr::new(
                        expr.span + rparen.span,
                        ExprKind::Call {
                            value: Box::new(expr),
                            arguments,
                        },
                    );
                }
                TokenType::LBrack => {
                    let index = self.parse_expr()?;
                    let rbrack = self.expect(TokenType::RBrack, "after the end of an index")?;
                    expr = Expr::new(
                        expr.span + rbrack.span,
                        ExprKind::Index {
                            value: Box::new(expr),
                            index: Box::new(index),
                        },
                    )
                }
                TokenType::Dot => {
                    let attr_token = self.maybe_identifier();
                    if let Some(string_span) = attr_token {
                        expr = Expr::new(
                            expr.span + string_span.1,
                            ExprKind::Attribute {
                                value: Box::new(expr),
                                attribute: string_span.0,
                            },
                        )
                    } else {
                        let tok = self.advance()?;
                        return Err(ParseError(
                            tok.span,
                            format!(
                                "Expected identifier after an attribute access, got {}",
                                tok.tty
                            ),
                        ));
                    }
                }
                _ => unreachable!(),
            };
        }
        Ok(expr)
    }

    fn parse_dice(&mut self) -> Result<Expr, ParseError> {
        let t = self.peek()?;
        let mut count: Option<Expr>;
        let mut do_dice = false;
        if let Some(_) = self.maybe_soft_keyword("d") {
            count = None;
            do_dice = true;
            self.advance();
        } else {
            count = Some(self.parse_primary()?);
            if let Some(_) = self.maybe_soft_keyword("d") {
                do_dice = true;
                self.advance();
            }
        };
        if do_dice {
            let sides: DiceSide;
            if let Some(_) = self.maybe(TokenType::Mod) {
                sides = DiceSide::Percent;
            } else {
                sides = DiceSide::Expr(Box::new(self.parse_primary()?));
            }
            Ok(count.unwrap())
        } else {
            Ok(count.unwrap()) // safe to to unwrap
        }
    }

    fn parse_primary(&mut self) -> Result<Expr, ParseError> {
        if let Some(string_span) = self.maybe_identifier() {
            return Ok(Expr {
                data: ExprKind::Variable(string_span.0),
                span: string_span.1,
            });
        }

        let token = self.advance()?;

        match token.tty {
            TokenType::Bool(b) => Ok(Expr::new(token.span, ExprKind::BoolLiteral(b))),
            TokenType::Int(i) => Ok(Expr::new(token.span, ExprKind::IntLiteral(i))),
            TokenType::Float(f) => Ok(Expr::new(token.span, ExprKind::FloatLiteral(f))),
            TokenType::None => Ok(Expr::new(token.span, ExprKind::None)),
            TokenType::String(s) => Ok(Expr::new(token.span, ExprKind::StringLiteral(s))),
            TokenType::LParen => {
                let inner = self.parse_expr()?;
                self.expect(TokenType::RParen, "to close a grouping")?;
                Ok(inner)
            }
            _ => Err(ParseError(token.span, format!("Unexpected {}", token.tty))),
        }
    }
}
