use crate::expr::dice::ast::Dice;

/*
Dice parsing is implemented a bit weirdly due to it needing to be called
by the main parser without passing in stuff like peek(), advance(), and etc.

Technically is this a regex? Finite-state automaton?
*/

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum ExplosionState {
    Bang,
    BangBang,
    BangP,
    BangBangP,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum DiceModPartialParseState {
    None,
    EncounteredMin,
    EncounteredMax,
    EncounteredExplosionFragment(ExplosionState),
    EncounteredR,
    EncounteredRO,
    EncounteredU,
    EncounteredUO,
    EncounteredK,
    EncounteredKL,
    EncounteredKH,
    EncounteredD,
    EncounteredDL,
    EncounteredDH,
    EncounteredF,
    EncounteredCS,
    EncounteredCF,
    EncounteredCSCF,
    
}

pub struct DiceParser {
    dice: Box<Dice>,
    state: DiceModPartialParseState,
}
