use std::collections::HashMap;

use crate::{interpreter::RuntimeError, literal_object::Literal, token::Token};

pub struct Environment {
    pub values: HashMap<String, Literal>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: &str, value: Literal) {
        self.values.insert(name.to_string(), value);
    }

    pub fn get(&self, name: Token) -> Result<Literal, RuntimeError> {
        if self.values.contains_key(&name.lexeme) {
            if let Some(literal) = self.values.get(&name.lexeme) {
                println!("getting literal: {:?}", literal);

                return Ok(literal.clone());
            } else {
                return Err(RuntimeError::new(
                    name.clone(),
                    &format!("Undefined variable '{}'.", name.lexeme),
                ));
            }
        }

        Err(RuntimeError::new(
            name.clone(),
            &format!("Undefined variable '{}'.", name.lexeme),
        ))
    }
}
