// src/runtime/environment.rs

use std::rc::Rc;
use std::collections::HashMap;
use crate::runtime::values::RuntimeValue;

#[derive(Clone)]
pub struct Environment {
    parent: Option<Box<Environment>>,
    variables: HashMap<String, Rc<dyn RuntimeValue>>,
}

impl Environment {
    pub fn new(parent_env: Option<Environment>) -> Self {
        Self {
            parent: parent_env.map(Box::new),
            variables: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Rc<dyn RuntimeValue>) {
        self.variables.insert(name, value);
    }
    
    pub fn assign(&mut self, name: String, value: Rc<dyn RuntimeValue>) {
        if let Some(env) = self.resolve(&name) {
            env.variables.insert(name, value);
        } else {
            // Handle the case when the variable is not found
            // For example, insert it into the current environment or raise an error
        }
    }

    pub fn lookup(&self, name: &str) -> Option<Rc<dyn RuntimeValue>> {
        self.variables.get(name)
            .cloned()
            .or_else(|| self.parent.as_ref()?.lookup(name))
    }

    pub fn resolve(&mut self, name: &str) -> Option<&mut Environment> {
        if self.variables.contains_key(name) {
            Some(self)
        } else {
            self.parent.as_mut().and_then(|parent| parent.resolve(name))
        }
    }
}
