use std::collections::HashMap;

use crate::expr::ty::types::Type;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SymbolID(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct SymbolTable {
    pub symbols: HashMap<SymbolID, SymbolType>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SymbolType {
    SingleType(Type),
    Overloaded(Vec<Type>),
}

impl SymbolTable {
    pub fn get_type(&self, sym: SymbolID) -> Option<&SymbolType> {
        self.symbols.get(&sym)
    }
}
