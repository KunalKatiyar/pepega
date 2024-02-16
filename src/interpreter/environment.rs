use std::collections::HashMap;
use crate::lexer::token::{LiteralValue, Token};

pub struct Environment {
    values: HashMap<String, LiteralValue>
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            values: HashMap::new()
        }
    }

    pub fn define(&mut self, name: String, value: LiteralValue) {
        self.values.insert(name, value);
    }

    pub fn assign(&mut self, name: &Token, value: LiteralValue) -> Result<(), String> {
        match self.values.get_mut(&name.lexeme) {
            Some(v) => {
                *v = value;
                Ok(())
            },
            None => Err(format!("Undefined variable '{}'.", name.lexeme))
        }
    }

    pub fn get(&self, name: &Token) -> Result<LiteralValue, String> {
        match self.values.get(&name.lexeme) {
            Some(value) => Ok(value.clone()),
            None => Err(format!("Undefined variable '{}'.", name.lexeme))
        }
    }
}