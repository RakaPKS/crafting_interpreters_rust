use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{error_reporter::RuntimeError, token::Literal};

#[derive(Debug, Clone)]
pub enum VariableState {
    Uninitialized,
    Initialized(Literal),
}

#[derive(Debug, Clone)]
pub struct Environment {
    inner: HashMap<String, VariableState>,
    outer: Option<SharedEnvironment>,
}

#[derive(Debug, Clone)]
pub struct SharedEnvironment(Rc<RefCell<Environment>>);

impl Environment {
    pub fn new(enclosing: Option<SharedEnvironment>) -> Self {
        Environment {
            inner: HashMap::new(),
            outer: enclosing,
        }
    }

    pub fn define(&mut self, identifier: String, value: Option<Literal>) {
        let state = match value {
            Some(lit) => VariableState::Initialized(lit),
            None => VariableState::Uninitialized,
        };
        self.inner.insert(identifier, state);
    }

    pub fn get(&self, identifier: &str) -> Result<Literal, RuntimeError> {
        match self.inner.get(identifier) {
            Some(VariableState::Initialized(value)) => Ok(value.clone()),
            Some(VariableState::Uninitialized) => Err(RuntimeError::UnInitializedVariable()),
            None => self
                .outer
                .as_ref()
                .map(|env| env.borrow().get(identifier))
                .unwrap_or_else(|| Err(RuntimeError::UndefinedVariable())),
        }
    }
    pub fn assign(&mut self, identifier: &str, value: Literal) -> Result<(), RuntimeError> {
        if self.inner.contains_key(identifier) {
            self.inner
                .insert(identifier.to_string(), VariableState::Initialized(value));
            Ok(())
        } else if let Some(outer) = &self.outer {
            outer.borrow_mut().assign(identifier, value)
        } else {
            Err(RuntimeError::UndefinedVariable())
        }
    }
}

impl SharedEnvironment {
    pub fn new(enclosing: Option<Self>) -> Self {
        Self(Rc::new(RefCell::new(Environment::new(enclosing))))
    }

    pub fn borrow(&self) -> std::cell::Ref<Environment> {
        self.0.borrow()
    }

    pub fn borrow_mut(&self) -> std::cell::RefMut<Environment> {
        self.0.borrow_mut()
    }
}
