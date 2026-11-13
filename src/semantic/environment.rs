
use std::collections::HashMap;

use crate::{Type};

use super::{Result, TypeError};

#[derive(Debug, Clone, PartialEq)]
pub struct TypeEnvironment {
    scopes: Vec<HashMap<String, Binding>>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Binding {
    pub ty: Type,
    pub mutable: bool,
}

impl TypeEnvironment {
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
        }
    }

    pub fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new())
    }

    pub fn exit_scope(&mut self) {
        assert!(self.scopes.len() > 1);
        self.scopes.pop();
    }

    pub fn define(
        &mut self,
        name: String,
        binding: Binding,
    ) -> Result<()> {
        let current = self.scopes.last_mut().unwrap();

        if current.contains_key(&name) {
            return Err(TypeError::DuplicateVariable(name));
        }

        current.insert(name, binding);
        Ok(())
    }

    pub fn lookup(&self, name: &str) -> Option<Binding> {
        self.scopes.iter()
            .rev()
            .find_map(|scope| scope.get(name).copied())
    }

    pub fn lookup_mut(&self, name: &str) -> Option<Binding> {
        self.lookup(name)
    }
}