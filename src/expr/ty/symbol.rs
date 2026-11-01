use std::collections::{HashMap, VecDeque};

use crate::expr::ty::types::Type;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SymbolID(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct SymbolTable {
    pub symbols: VecDeque<HashMap<SymbolID, SymbolType>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SymbolType {
    SingleType(Type),
    Overloaded(Vec<Type>),
}

impl SymbolTable {
    pub fn new(symbols: HashMap<SymbolID, SymbolType>) -> SymbolTable {
        SymbolTable {
            symbols: VecDeque::from([symbols]),
        }
    }

    pub fn get_type(&self, sym: SymbolID) -> Option<&SymbolType> {
        let mut index = self.symbols.len() - 1;
        loop {
            if let Some(hashmap) = self.symbols.get(index) {
                if let Some(symbol_type) = hashmap.get(&sym) {
                    break Some(symbol_type);
                } else {
                    index -= 1;
                }
            } else {
                break None;
            }
        }
    }

    pub fn insert(&mut self, sym: SymbolID, sym_type: SymbolType) -> &mut SymbolTable {
        if let Some(layer) = self.symbols.get_mut(self.symbols.len() - 1) {
            layer.insert(sym, sym_type);
        } else {
            self.symbols.push_back(HashMap::from([(sym, sym_type)]));
        }
        self
    }

    pub fn push_layer(&mut self) -> &mut SymbolTable {
        self.symbols.push_back(HashMap::with_capacity(1));
        self
    }

    pub fn pop_layer(&mut self) -> &mut SymbolTable {
        self.symbols.pop_back();
        self
    }
}
