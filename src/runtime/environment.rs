// src/runtime/environment.rs

use std::rc::Rc;
use std::collections::{HashMap, HashSet};
use crate::runtime::values::RuntimeValue;

#[derive(Clone)]
pub struct Environment {
    parent: Option<Box<Environment>>,
    variables: HashMap<String, Rc<dyn RuntimeValue>>,
    constants: HashSet<String>,
}

impl Environment {
    pub fn new(parent_env: Option<Environment>) -> Self {
        Self {
            parent: parent_env.map(Box::new),
            variables: HashMap::new(),
            constants: HashSet::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Rc<dyn RuntimeValue>, constant: bool) {
        if self.variables.contains_key(&name) || self.constants.contains(&name) {
            panic!("Variable {} already defined", name);
        }

        if constant {
            self.constants.insert(name.clone());
        }
        self.variables.insert(name, value);
    }
    
    pub fn assign(&mut self, name: String, value: Rc<dyn RuntimeValue>) {
        if self.constants.contains(&name) {
            panic!("Cannot reassign a constant {}", name);
        }

        if let Some(variable) = self.variables.get_mut(&name) {
            *variable = value;
        } else if let Some(parent) = self.parent.as_mut() {
            parent.assign(name, value);
        } else {
            // Variable not found in the current environment and has no parent
            panic!("Undefined variable {}", name);
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
