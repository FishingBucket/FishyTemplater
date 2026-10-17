use crate::expr::ast::{CompOp, Expr};

#[derive(Debug, Clone, PartialEq)]
pub enum DiceSide {
    Percent,
    Expr(Box<Expr>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Sort {
    Ascending,
    Descending,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ComparePoint {
    pub operator: CompOp,
    pub value: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ModMin {
    pub min: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ModMax {
    pub max: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Explode {
    pub cp: Option<ComparePoint>,
    pub compound: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Penetrate {
    pub cp: Option<ComparePoint>,
    pub compound: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Reroll {
    pub cp: Option<ComparePoint>,
    pub once: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Unique {
    pub cp: Option<ComparePoint>,
    pub once: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct KeepHigh {
    pub number: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct KeepLow {
    pub number: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DropHigh {
    pub number: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DropLow {
    pub number: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Success {
    pub cp: ComparePoint,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Failure {
    pub cp: ComparePoint,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CriticalSuccess {
    pub cp: Option<ComparePoint>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CriticalFailure {
    pub cp: Option<ComparePoint>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Dice {
    count: Option<Box<Expr>>,
    side: DiceSide,
    min: Option<ModMin>,
    max: Option<ModMax>,
    explode: Option<Explode>,
    penetrate: Option<Penetrate>,
    reroll: Option<Reroll>,
    unique: Option<Unique>,
    keep_high: Option<KeepHigh>,
    keep_low: Option<KeepLow>,
    drop_high: Option<DropHigh>,
    drop_low: Option<DropLow>,
    success: Option<Success>,
    failure: Option<Failure>,
    critical_success: Option<CriticalSuccess>,
    critical_failure: Option<CriticalFailure>,
    sort: Option<Sort>,
}
