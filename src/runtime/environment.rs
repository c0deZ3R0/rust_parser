// src/runtime/environment.rs

use crate::runtime::values::{RuntimeValue,makebool};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;



#[derive(Clone)]
pub struct Environment {
	parent: Option<Box<Environment>>,
	variables: HashMap<String, Rc<dyn RuntimeValue>>,
	constants: HashSet<String>,
}

impl Environment {
	pub fn new(parent_env: Option<Environment>) -> Self {

		let mut environment = Self {
            parent: parent_env.map(Box::new),
            variables: HashMap::new(),
            constants: HashSet::new(),
        };

			      // Setup the scope only if there is no parent environment
				if environment.parent.is_none() {
					environment.setup_scope();
					//TODO: Pretty sure this is wrong and wont work if we have a parent environment
				};
		

				environment
		
	}

    fn setup_scope(&mut self) {
        // Define default variables and constants here
        // For example, defining a boolean constant
        self.define("true".to_string(), makebool(Some(true)), true);
        self.define("false".to_string(), makebool(Some(false)), true);
        // Add other default values as needed
    }

	pub fn define(
		&mut self,
		name: String,
		value: Rc<dyn RuntimeValue>,
		constant: bool,
	) {
		
		if self.variables.contains_key(&name) || self.constants.contains(&name) {
			panic!("Variable {} already defined", name);
			//TODO:Define error type for this
		}

		if constant {
			self.constants.insert(name.clone());
		}
		self.variables.insert(name, value);
	}

	pub fn assign(&mut self, name: String, value: Rc<dyn RuntimeValue>) {
		if self.constants.contains(&name) {
			panic!("Cannot reassign a constant {}", name);
			//TODO:Define error type for this
		}

		if let Some(variable) = self.variables.get_mut(&name) {
			*variable = value;
		} else if let Some(parent) = self.parent.as_mut() {
			parent.assign(name, value);
		} else {
			// Variable not found in the current environment and has no parent
			panic!("Undefined variable {}", name);
			//TODO:Define error type for this
		}
	}

	pub fn lookup(&self, name: &str) -> Option<Rc<dyn RuntimeValue>> {
		self.variables
			.get(name)
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
