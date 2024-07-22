use std::collections::HashMap;

use crate::{error_reporter::RuntimeError, token::Literal};

#[derive(Debug, Clone)]
pub struct Environment {
    scopes: Vec<HashMap<String, VariableState>>,
}

#[derive(Debug, Clone)]
pub enum VariableState {
    Uninitialized,
    Initialized(Literal),
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            scopes: vec![HashMap::new()],
        }
    }

    pub fn increase_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn reduce_scope(&mut self) -> Result<(), RuntimeError> {
        if self.scopes.len() > 1 {
            self.scopes.pop();
            Ok(())
        } else {
            Err(RuntimeError::CannotReduceGlobalScope)
        }
    }

    pub fn define(&mut self, identifier: String, value: Option<Literal>) {
        let state = match value {
            Some(lit) => VariableState::Initialized(lit),
            None => VariableState::Uninitialized,
        };
        self.scopes.last_mut().unwrap().insert(identifier, state);
    }

    pub fn get(&self, identifier: &str) -> Result<Literal, RuntimeError> {
        for scope in self.scopes.iter().rev() {
            if let Some(state) = scope.get(identifier) {
                match state {
                    VariableState::Initialized(value) => return Ok(value.clone()),
                    VariableState::Uninitialized => {
                        return Err(RuntimeError::UnInitializedVariable)
                    }
                }
            }
        }
        Err(RuntimeError::UndefinedVariable)
    }

    pub fn assign(&mut self, identifier: &str, value: Literal) -> Result<(), RuntimeError> {
        for scope in self.scopes.iter_mut().rev() {
            if scope.contains_key(identifier) {
                scope.insert(identifier.to_string(), VariableState::Initialized(value));
                return Ok(());
            }
        }
        Err(RuntimeError::UndefinedVariable)
    }
}

