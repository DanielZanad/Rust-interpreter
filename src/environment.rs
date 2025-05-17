use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{interpreter::RuntimeError, literal_object::Literal, token::Token};

#[derive(Clone)]
pub struct Environment {
    pub values: HashMap<String, Literal>,
    pub enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new(enclosing: Option<Rc<RefCell<Environment>>>) -> Self {
        Environment {
            values: HashMap::new(),
            enclosing,
        }
    }

    pub fn new_enclosed(enclosing: Rc<RefCell<Environment>>) -> Self {
        Environment {
            values: HashMap::new(),
            enclosing: Some(enclosing),
        }
    }

    pub fn define(&mut self, name: &str, value: Literal) {
        self.values.insert(name.to_string(), value);
    }

    pub fn assign(&mut self, name: Token, value: Literal) -> Result<(), RuntimeError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value);
            return Ok(());
        }

        if let Some(ref enclosing) = self.enclosing {
            return enclosing.borrow_mut().assign(name, value);
        }

        Err(RuntimeError::new(
            name.clone(),
            &format!("Undefined variable '{}'.", name.lexeme),
        ))
    }

    pub fn get(&self, name: Token) -> Result<Literal, RuntimeError> {
        if let Some(val) = self.values.get(&name.lexeme) {
            return Ok(val.clone());
        }

        if let Some(ref enclosing) = self.enclosing {
            return enclosing.borrow().get(name);
        }

        Err(RuntimeError::new(
            name.clone(),
            &format!("Undefined variable '{}'.", name.lexeme),
        ))
    }
}
