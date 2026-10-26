use crate::expr::{
    ast::{CompOp, Expr}, dice::ast::{
        ComparePoint, Dice, DiceSide, ModCriticalSuccessFail, ModExplodePenetrate, ModKeepDrop, ModMinMax, ModRerollUnique, ModSort, ModSuccessFail
    }, parse::{
        parser::ParseError,
        tokens::{IdentType, Token, TokenType},
    },
};

/*
Dice parsing is implemented a bit weirdly due to it needing to be called
by the main parser without passing in stuff like peek(), advance(), and etc.

Technically is this a regex? Finite-state automaton?
*/

#[derive(Clone, Copy, PartialEq, Eq)]
enum DiceModState {
    None,
    Min,
    Max,
    Bang,
    BangBang,
    BangP,
    BangBangP,
    Reroll,
    RerollOnce,
    Unique,
    UniqueOnce,
    KeepLow,
    KeepHigh,
    DropLow,
    DropHigh,
    Fail,
    CriticalSuccess,
    CriticalFailure,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiceParserNextState {
    Quit,
    Advance,
    AdvanceNextExprIn,
}

pub struct DiceParser {
    dice: Dice,
    state: DiceModState,
    compare_point: Option<TokenType>,
}

impl DiceParser {
    pub fn new(count: Option<Expr>, side: DiceSide) -> DiceParser {
        DiceParser {
            dice: Dice::new(count, side),
            state: DiceModState::None,
            compare_point: None,
        }
    }

    pub fn get_dice(self) -> Dice {
        self.dice
    }

    pub fn in_token(&mut self, token: &Token) -> Result<DiceParserNextState, ParseError> {
        if let TokenType::Ident(IdentType::Split(d) | IdentType::Full(d)) = &token.tty {
            match d.as_str() {
                "min" => {
                    self.state = DiceModState::Min;
                    Ok(DiceParserNextState::AdvanceNextExprIn)
                }
                "max" => {
                    self.state = DiceModState::Max;
                    Ok(DiceParserNextState::AdvanceNextExprIn)
                }
                "r" => {
                    self.state = DiceModState::Reroll;
                    self.dice.reroll = Some(ModRerollUnique {
                        cp: None,
                        once: false,
                    });
                    Ok(DiceParserNextState::Advance)
                }
                "ro" => {
                    self.state = DiceModState::RerollOnce;
                    self.dice.reroll = Some(ModRerollUnique {
                        cp: None,
                        once: true,
                    });
                    Ok(DiceParserNextState::Advance)
                }
                "u" => {
                    self.state = DiceModState::Unique;
                    self.dice.unique = Some(ModRerollUnique {
                        cp: None,
                        once: false,
                    });
                    Ok(DiceParserNextState::Advance)
                }
                "uo" => {
                    self.state = DiceModState::UniqueOnce;
                    self.dice.unique = Some(ModRerollUnique {
                        cp: None,
                        once: true,
                    });
                    Ok(DiceParserNextState::Advance)
                }
                "k" | "kh" => {
                    self.state = DiceModState::KeepHigh;
                    Ok(DiceParserNextState::AdvanceNextExprIn)
                }
                "kl" => {
                    self.state = DiceModState::KeepLow;
                    Ok(DiceParserNextState::AdvanceNextExprIn)
                }
                "d" | "dl" => {
                    self.state = DiceModState::DropLow;
                    Ok(DiceParserNextState::AdvanceNextExprIn)
                }
                "dh" => {
                    self.state = DiceModState::DropHigh;
                    Ok(DiceParserNextState::AdvanceNextExprIn)
                }
                "f" => {
                    self.state = DiceModState::Fail;
                    Ok(DiceParserNextState::Advance)
                }
                "cs" => {
                    self.state = DiceModState::CriticalSuccess;
                    self.dice.critical_success = Some(ModCriticalSuccessFail { cp: None });
                    Ok(DiceParserNextState::Advance)
                }
                "cf" => {
                    self.state = DiceModState::CriticalFailure;
                    self.dice.critical_failure = Some(ModCriticalSuccessFail { cp: None });
                    Ok(DiceParserNextState::Advance)
                }
                "s" => {
                    self.state = DiceModState::None;
                    self.dice.sort = Some(ModSort::Ascending);
                    Ok(DiceParserNextState::Advance)
                }
                "sa" => {
                    self.state = DiceModState::None;
                    self.dice.sort = Some(ModSort::Ascending);
                    Ok(DiceParserNextState::Advance)
                }
                "sd" => {
                    self.state = DiceModState::None;
                    self.dice.sort = Some(ModSort::Descending);
                    Ok(DiceParserNextState::Advance)
                }
                "p" => {
                    if self.state == DiceModState::Bang {
                        self.state = DiceModState::BangP;
                        self.dice.explode = None;
                        self.dice.penetrate = Some(ModExplodePenetrate {
                            cp: None,
                            compound: false,
                        });
                        Ok(DiceParserNextState::Advance)
                    } else if self.state == DiceModState::BangBang {
                        self.state = DiceModState::BangBangP;
                        self.dice.explode = None;
                        self.dice.penetrate = Some(ModExplodePenetrate {
                            cp: None,
                            compound: true,
                        });
                        Ok(DiceParserNextState::Advance)
                    } else {
                        Ok(DiceParserNextState::Quit)
                    }
                }
                _ => Ok(DiceParserNextState::Quit),
            }
        } else if token.tty == TokenType::Not {
            if self.state == DiceModState::Bang {
                self.dice.explode = Some(ModExplodePenetrate {
                    cp: None,
                    compound: true,
                });
                self.state = DiceModState::BangBang;
                Ok(DiceParserNextState::Advance)
            } else {
                self.dice.explode = Some(ModExplodePenetrate {
                    cp: None,
                    compound: false,
                });
                self.state = DiceModState::Bang;
                Ok(DiceParserNextState::Advance)
            }
        } else {
            let got_cp = self.maybe_compare_point(token)?;
            if self.state == DiceModState::Fail && !got_cp {
                Err(ParseError(
                    token.span,
                    format!("Expected compare point, got {}", token.tty),
                ))
            } else if got_cp {
                Ok(DiceParserNextState::AdvanceNextExprIn)
            } else {
                Ok(DiceParserNextState::Advance)
            }
        }
    }

    pub fn in_expr(&mut self, expr: Expr) -> Result<DiceParserNextState, ParseError> {
        match self.state {
            DiceModState::None => self.dice.success = Some(ModSuccessFail { cp: self.cp(expr)? }),
            DiceModState::Min => {
                self.dice.min = Some(ModMinMax {
                    target: Box::new(expr),
                })
            }
            DiceModState::Max => {
                self.dice.max = Some(ModMinMax {
                    target: Box::new(expr),
                })
            }
            DiceModState::Bang => {
                self.dice.explode = Some(ModExplodePenetrate {
                    cp: Some(self.cp(expr)?),
                    compound: false,
                })
            }
            DiceModState::BangBang => {
                self.dice.explode = Some(ModExplodePenetrate {
                    cp: Some(self.cp(expr)?),
                    compound: true,
                })
            }
            DiceModState::BangP => {
                self.dice.penetrate = Some(ModExplodePenetrate {
                    cp: Some(self.cp(expr)?),
                    compound: false,
                })
            }
            DiceModState::BangBangP => {
                self.dice.penetrate = Some(ModExplodePenetrate {
                    cp: Some(self.cp(expr)?),
                    compound: true,
                })
            }
            DiceModState::Reroll => {
                self.dice.reroll = Some(ModRerollUnique {
                    cp: Some(self.cp(expr)?),
                    once: false,
                })
            }
            DiceModState::RerollOnce => {
                self.dice.reroll = Some(ModRerollUnique {
                    cp: Some(self.cp(expr)?),
                    once: true,
                })
            }
            DiceModState::Unique => {
                self.dice.unique = Some(ModRerollUnique {
                    cp: Some(self.cp(expr)?),
                    once: false,
                })
            }
            DiceModState::UniqueOnce => {
                self.dice.unique = Some(ModRerollUnique {
                    cp: Some(self.cp(expr)?),
                    once: true,
                })
            }
            DiceModState::KeepHigh => {
                self.dice.keep_high = Some(ModKeepDrop {
                    number: Box::new(expr),
                })
            }
            DiceModState::KeepLow => {
                self.dice.keep_low = Some(ModKeepDrop {
                    number: Box::new(expr),
                })
            }
            DiceModState::DropLow => {
                self.dice.drop_low = Some(ModKeepDrop {
                    number: Box::new(expr),
                })
            }
            DiceModState::DropHigh => {
                self.dice.drop_high = Some(ModKeepDrop {
                    number: Box::new(expr),
                })
            }
            DiceModState::Fail => self.dice.failure = Some(ModSuccessFail { cp: self.cp(expr)? }),
            DiceModState::CriticalSuccess => {
                self.dice.critical_success = Some(ModCriticalSuccessFail {
                    cp: Some(self.cp(expr)?),
                })
            }
            DiceModState::CriticalFailure => {
                self.dice.critical_failure = Some(ModCriticalSuccessFail {
                    cp: Some(self.cp(expr)?),
                })
            }
        };
        self.state = DiceModState::None;
        Ok(DiceParserNextState::Advance)
    }
}

impl DiceParser {
    fn cp(&mut self, expr: Expr) -> Result<ComparePoint, ParseError> {
        if let Some(cp) = &self.compare_point {
            let res = ComparePoint {
                operator: match cp {
                    TokenType::Eq => CompOp::Equals,
                    TokenType::Eqeq => CompOp::Equals,
                    TokenType::Neq => CompOp::NotEquals,
                    TokenType::Neq2 => CompOp::NotEquals,
                    TokenType::Gt => CompOp::Greater,
                    TokenType::Geq => CompOp::GreaterEquals,
                    TokenType::Lt => CompOp::Less,
                    TokenType::Leq => CompOp::LessEquals,
                    _ => unreachable!(),
                },
                value: Box::new(expr),
            };
            self.compare_point = None;
            Ok(res)
        } else {
            Err(ParseError(
                expr.span,
                "Expected comparison before expression".to_string(),
            ))
        }
    }

    fn maybe_compare_point(&mut self, token: &Token) -> Result<bool, ParseError> {
        if self.compare_point.is_some() {
            return Err(ParseError(
                token.span,
                format!("Expected expression for compare point, got {}", token.tty),
            ));
        }
        match &token.tty {
            TokenType::Eq
            | TokenType::Eqeq
            | TokenType::Neq
            | TokenType::Neq2
            | TokenType::Gt
            | TokenType::Geq
            | TokenType::Lt
            | TokenType::Leq => {
                self.compare_point = Some(token.tty.clone());
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}
