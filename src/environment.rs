use std::collections::HashMap;

use crate::{
    interpreter::RuntimeError,
    token::{Literal, Token},
};

pub struct Environment {
    values: HashMap<String, Literal>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
        }
    }

    pub fn get(&self, name: &Token) -> Result<&Literal, RuntimeError> {
        dbg!(&self.values.get(&name.lexeme));
        self.values.get(&name.lexeme).ok_or(RuntimeError {
            message: format!("Undefined variable \"{}\".", name.lexeme),
            line: name.line,
        })
    }

    pub fn define(&mut self, name: String, value: Literal) -> Result<(), RuntimeError> {
        dbg!(&name, &value);
        self.values.insert(name, value);
        Ok(())
    }
}
