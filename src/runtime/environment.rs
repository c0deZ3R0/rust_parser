use std::collections::HashMap;
use crate::runtime::values::RuntimeVal;

#[derive(Clone)]
pub struct Environment {
    parent: Box<Option<Environment>>,
    variables: HashMap<String, Box<dyn RuntimeVal>>,
}

impl Environment {
    pub fn new(parent_env: Option<Environment>) -> Self {
        Self {
            parent: Box::new(parent_env),
            variables: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Box<dyn RuntimeVal>) {
        self.variables.insert(name, value);
    }

    
    pub fn assign(&mut self, name: String, value: Box<dyn RuntimeVal>) {
        if self.variables.contains_key(&name) {
            self.variables.insert(name, value);
        } else {
            match self.parent.as_ref() {
                Some(parent) => parent.assign(name, value),
                None => panic!("Undefined variable {}", name),
            }
        }   
    }

    pub fn resolve(&self, name: &str) -> &Environment {
        if self.variables.contains_key(name) {
            self
        } else {
            match self.parent.as_ref() {
                Some(parent) => parent.resolve(name),
                None => panic!("Undefined variable {}", name),
            }
        }
    } 


}
