use crate::expr::ast::{CompOp, Expr};

#[derive(Debug, Clone, PartialEq)]
pub enum DiceSide {
    Percent,
    Expr { expr: Box<Expr> },
}

#[derive(Debug, Clone, PartialEq)]
pub enum ModSort {
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
pub struct ModExplode {
    pub cp: Option<ComparePoint>,
    pub compound: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ModPenetrate {
    pub cp: Option<ComparePoint>,
    pub compound: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ModReroll {
    pub cp: Option<ComparePoint>,
    pub once: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ModUnique {
    pub cp: Option<ComparePoint>,
    pub once: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ModKeepHigh {
    pub number: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ModKeepLow {
    pub number: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ModDropHigh {
    pub number: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ModDropLow {
    pub number: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ModSuccess {
    pub cp: ComparePoint,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ModFailure {
    pub cp: ComparePoint,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ModCriticalSuccess {
    pub cp: Option<ComparePoint>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ModCriticalFailure {
    pub cp: Option<ComparePoint>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Dice {
    pub count: Box<Option<Expr>>,
    pub side: DiceSide,
    pub min: Option<ModMin>,
    pub max: Option<ModMax>,
    pub explode: Option<ModExplode>,
    pub penetrate: Option<ModPenetrate>,
    pub reroll: Option<ModReroll>,
    pub unique: Option<ModUnique>,
    pub keep_high: Option<ModKeepHigh>,
    pub keep_low: Option<ModKeepLow>,
    pub drop_high: Option<ModDropHigh>,
    pub drop_low: Option<ModDropLow>,
    pub success: Option<ModSuccess>,
    pub failure: Option<ModFailure>,
    pub critical_success: Option<ModCriticalSuccess>,
    pub critical_failure: Option<ModCriticalFailure>,
    pub sort: Option<ModSort>,
}

impl Dice {
    pub fn new(count: Option<Expr>, side: DiceSide) -> Dice {
        Dice {
            count: Box::new(count),
            side,
            min: None,
            max: None,
            explode: None,
            penetrate: None,
            reroll: None,
            unique: None,
            keep_high: None,
            keep_low: None,
            drop_high: None,
            drop_low: None,
            success: None,
            failure: None,
            critical_success: None,
            critical_failure: None,
            sort: None,
        }
    }
}
