use std::collections::HashMap;
use crate::lexer::token::{LiteralValue, Token};

pub struct Environment {
    enclosing: Option<Box<Environment>>,
    values: HashMap<String, LiteralValue>
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            enclosing: None,
            values: HashMap::new()
        }
    }

    pub fn define(&mut self, name: String, value: LiteralValue) {
        self.values.insert(name, value);
    }

    pub fn clone(&self) -> Environment {
        let enclosing = self.enclosing.as_ref().map(|e| Box::new((*e).clone()));

        Environment {
            enclosing,
            values: self.values.clone(),
        }
    }

    pub fn assign(&mut self, name: &Token, value: LiteralValue) -> Result<(), String> {
        match self.values.get_mut(&name.lexeme) {
            Some(v) => {
                *v = value;
                Ok(())
            },
            None => {
                match self.enclosing {
                    Some(ref mut e) => e.assign(name, value),
                    None => Err(format!("Undefined variable '{}'.", name.lexeme))
                }
            }
        }
    }

    pub fn get(&self, name: &Token) -> Result<LiteralValue, String> {
        match self.values.get(&name.lexeme) {
            Some(v) => Ok(v.clone()),
            None => {
                match self.enclosing {
                    Some(ref e) => e.get(name),
                    None => Err(format!("Undefined variable '{}'.", name.lexeme))
                }
            }
        }
    }

    pub fn new_with_enclosing(enclosing: Environment) -> Environment {
        Environment {
            enclosing: Some(Box::new(enclosing)),
            values: HashMap::new()
        }
    }
}