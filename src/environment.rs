use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{error_reporter::RuntimeError, token::Literal};

#[derive(Clone, Debug)]
pub enum VariableState {
    Uninitialized,
    Initialized(Literal),
}

#[derive(Clone, Debug)]
pub struct Environment {
    inner: HashMap<String, VariableState>,
    outer: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new(enclosing: Option<Rc<RefCell<Environment>>>) -> Self {
        Environment {
            inner: HashMap::new(),
            outer: enclosing,
        }
    }

    pub fn define(&mut self, name: String, value: Option<Literal>) {
        let state = match value {
            Some(lit) => VariableState::Initialized(lit),
            None => VariableState::Uninitialized,
        };
        self.inner.insert(name, state);
    }

    pub fn get(&self, name: &str) -> Option<Literal> {
        match self.inner.get(name) {
            Some(VariableState::Initialized(value)) => Some(value.clone()),
            Some(VariableState::Uninitialized) => Some(Literal::Nil), // Or handle uninitialized variables as needed
            None => self.outer.as_ref().and_then(|out| out.borrow().get(name)),
        }
    }
    pub fn assign(&mut self, name: &str, value: Literal) -> Result<(), RuntimeError> {
        if self.inner.contains_key(name) {
            self.inner
                .insert(name.to_string(), VariableState::Initialized(value));
            Ok(())
        } else if let Some(outer) = &self.outer {
            outer.borrow_mut().assign(name, value)
        } else {
            Err(RuntimeError::UndefinedVariable())
        }
    }
}
