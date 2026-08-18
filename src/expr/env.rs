use std::{collections::HashMap};

use derive_more::Display;

use crate::expr::ty::TypeArena;

use super::{ty::Type, value::Value};

pub struct Environment {
    pub variables: VariableTable,
    pub types: TypeArena
}

struct VariableScope(HashMap<String, VariableInfo>);

impl VariableScope {
    pub fn new() -> VariableScope {
        VariableScope(HashMap::new())
    }

    pub fn insert(&mut self, name: String, info: VariableInfo) {
        self.0.insert(name, info);
    }

    pub fn get(&self, name: &String) -> Option<&VariableInfo> {
        self.0.get(name)
    }
}

struct VariableTable(Vec<VariableScope>);

impl VariableTable {
    pub fn new() -> VariableTable {
        VariableTable(vec![VariableScope::new()])
    }

    pub fn insert(&mut self, name: String, info: VariableInfo) {
        if let Some(scope) = self.0.last_mut() {
            scope.insert(name, info);
        }
    }

    pub fn get(&self, name: &String) -> Option<&VariableInfo> {
        for scope in self.0.iter().rev() {
            if let Some(info) = scope.get(name) {
                return Some(info);
            }
        }
        None
    }

    pub fn push(&mut self, scope: VariableScope) {
        self.0.push(scope);
    }

    pub fn pop(&mut self) -> Option<VariableScope> {
        self.0.pop()
    }
}

#[derive(Display)]
#[display("VariableInfo({ty}, {mutable}, {value})")]
struct VariableInfo {
    pub ty: Type,
    pub mutable: bool,
    pub value: Box<dyn Value>
}