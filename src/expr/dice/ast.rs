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
pub struct ModMinMax {
    pub target: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ModExplodePenetrate {
    pub cp: Option<ComparePoint>,
    pub compound: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ModRerollUnique {
    pub cp: Option<ComparePoint>,
    pub once: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ModKeepDrop {
    pub number: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ModSuccessFail {
    pub cp: ComparePoint,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ModCriticalSuccessFail {
    pub cp: Option<ComparePoint>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Dice {
    pub count: Box<Option<Expr>>,
    pub side: DiceSide,
    pub min: Option<ModMinMax>,
    pub max: Option<ModMinMax>,
    pub explode: Option<ModExplodePenetrate>,
    pub penetrate: Option<ModExplodePenetrate>,
    pub reroll: Option<ModRerollUnique>,
    pub unique: Option<ModRerollUnique>,
    pub keep_high: Option<ModKeepDrop>,
    pub keep_low: Option<ModKeepDrop>,
    pub drop_high: Option<ModKeepDrop>,
    pub drop_low: Option<ModKeepDrop>,
    pub success: Option<ModSuccessFail>,
    pub failure: Option<ModSuccessFail>,
    pub critical_success: Option<ModCriticalSuccessFail>,
    pub critical_failure: Option<ModCriticalSuccessFail>,
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
