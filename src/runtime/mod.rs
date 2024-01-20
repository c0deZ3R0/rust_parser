pub mod environment;
pub mod values;

use crate::parser::{self, Program};
use crate::tokens::TokenValue::{BinaryExpr, Identifier, Number, VarDeclaration};
use crate::tokens::{TokenType, TokenValue};
use std::env::var;
use std::rc::Rc;

use values::{NullVal, NumberVal, RuntimeVal, RuntimeValue, ValueType};

use self::environment::Environment;
use self::values::makenull;

//TODO:Define error type for this
pub struct Interpreter {
	ast: Program,
	env: Environment,
}

impl Interpreter {
	pub fn new(ast: Program) -> Self {
		Self {
			ast,
			env: Environment::new(None),
		}
	}

	pub fn eval_program(&mut self, env: &mut Environment) -> Rc<dyn RuntimeValue> {
		// Iterate over self.ast.body and evaluate each expression using the provided env
		self.ast
			.body
			.iter()
			.map(|expr| self.eval(expr, env)) // Pass the mutable reference directly
			.last()
			.unwrap_or_else(|| Rc::new(NullVal))
	}

	fn eval(
		&self,
		token: &TokenValue,
		env: &mut Environment,
	) -> Rc<dyn RuntimeValue> {
		match token {
			TokenValue::Number(n) => Rc::new(NumberVal::new(*n)),
			TokenValue::Identifier(name) => self.iden(name, env),
			TokenValue::AssignmentExpr(expr, value) => self.eval_assignment(token, expr, value, env),
			TokenValue::BinaryExpr(left, right, op) => {
				let left_val = self.eval(left, env);
				let right_val = self.eval(right, env);

				if left_val.get_type() == ValueType::Null
					|| right_val.get_type() == ValueType::Null
				{
					return Rc::new(NullVal);
				}

				match op {
					TokenType::Plus => self.add(left_val, right_val),
					TokenType::Minus => self.sub(left_val, right_val),
					TokenType::Times => self.mul(left_val, right_val),
					TokenType::Divide => self.div(left_val, right_val),
					// Handle other operators
					_ => unimplemented!(),
				}
			}
			TokenValue::VarDeclaration(name, is_const, expr) => {
			
				println!("{} {} {:?}", name, is_const, expr);
				self.vardec(name, *is_const, expr, env)

				
			},
			

			_ => unimplemented!(),
		}
	}


	fn eval_assignment(
		&self,
		handler: &TokenValue,
		expr: &TokenValue,
		value: &TokenValue,
		env: &mut Environment,
	) -> Rc<dyn RuntimeValue> {

		

		match handler{
			TokenValue::AssignmentExpr(_,_) =>
			{
				let varname = match expr {
					TokenValue::Identifier(name) => name,
					_ => unimplemented!(),
				};

				let evaluated_value = self.eval(value, env);

				env.assign(varname.to_string(), evaluated_value.clone());
				evaluated_value
			}
			_ => unimplemented!(),		}
			//TODO:Define error type for this
		}
	
	
	
	


	fn vardec(
		&self,
		name: &str,
		is_const: bool,
		expr: &TokenValue,
		env: &mut Environment,
	) -> Rc<dyn RuntimeValue> {
		
		
		match expr {	
			TokenValue::Null => {
				env.define(name.to_string(), Rc::new(NullVal), is_const);
				return Rc::new(NullVal);
			}

			
			_ => (),

		}

		let value = match expr {
			expr_rc => self.eval(expr_rc, env),
			
		};
		

		env.define(name.to_string(), value.clone(), is_const);
		value
	}

	
	
	
	
	fn iden(&self, iden: &String, env: &Environment) -> Rc<dyn RuntimeValue> {
		match env.lookup(iden) {
			Some(val) => val.clone(),
			None => Rc::new(NullVal),
		}
	}

	

	fn sub(
		&self,
		left: Rc<dyn RuntimeValue>,
		right: Rc<dyn RuntimeValue>,
	) -> Rc<dyn RuntimeValue> {
		match (left.as_ref(), right.as_ref()) {
			(left_val, right_val) => {
				// Check if both values are NumberVal
				if left_val.get_type() == ValueType::Number
					&& right_val.get_type() == ValueType::Number
				{
					// Perform addition
					let left_number = left_val
						.as_any()
						.downcast_ref::<NumberVal>()
						.expect("Type mismatch");
					let right_number = right_val
						.as_any()
						.downcast_ref::<NumberVal>()
						.expect("Type mismatch");
					Rc::new(NumberVal::new(
						left_number.value() - right_number.value(),
					))
				} else {
					// Handle other types or errors
					unimplemented!()
				}
			}
		}
	}

	fn add(
		&self,
		left: Rc<dyn RuntimeValue>,
		right: Rc<dyn RuntimeValue>,
	) -> Rc<dyn RuntimeValue> {
		match (left.as_ref(), right.as_ref()) {
			(left_val, right_val) => {
				// Check if both values are NumberVal
				if left_val.get_type() == ValueType::Number
					&& right_val.get_type() == ValueType::Number
				{
					// Perform addition
					let left_number = left_val
						.as_any()
						.downcast_ref::<NumberVal>()
						.expect("Type mismatch");
					let right_number = right_val
						.as_any()
						.downcast_ref::<NumberVal>()
						.expect("Type mismatch");
					Rc::new(NumberVal::new(
						left_number.value() + right_number.value(),
					))
				} else {
					// Handle other types or errors
					unimplemented!()
				}
			}
		}
	}

	fn mul(
		&self,
		left: Rc<dyn RuntimeValue>,
		right: Rc<dyn RuntimeValue>,
	) -> Rc<dyn RuntimeValue> {
		match (left.as_ref(), right.as_ref()) {
			(left_val, right_val) => {
				// Check if both values are NumberVal
				if left_val.get_type() == ValueType::Number
					&& right_val.get_type() == ValueType::Number
				{
					// Perform addition
					let left_number = left_val
						.as_any()
						.downcast_ref::<NumberVal>()
						.expect("Type mismatch");
					let right_number = right_val
						.as_any()
						.downcast_ref::<NumberVal>()
						.expect("Type mismatch");
					Rc::new(NumberVal::new(
						left_number.value() * right_number.value(),
					))
				} else {
					// Handle other types or errors
					unimplemented!()
				}
			}
		}
	}

	fn div(
		&self,
		left: Rc<dyn RuntimeValue>,
		right: Rc<dyn RuntimeValue>,
	) -> Rc<dyn RuntimeValue> {
		match (left.as_ref(), right.as_ref()) {
			(left_val, right_val) => {
				// Check if both values are NumberVal
				if left_val.get_type() == ValueType::Number
					&& right_val.get_type() == ValueType::Number
				{
					// Perform addition
					let left_number = left_val
						.as_any()
						.downcast_ref::<NumberVal>()
						.expect("Type mismatch");
					let right_number = right_val
						.as_any()
						.downcast_ref::<NumberVal>()
						.expect("Type mismatch");
					Rc::new(NumberVal::new(
						left_number.value() / right_number.value(),
					))
				} else {
					// Handle other types or errors
					unimplemented!()
				}
			}
		}
	}
}


#[cfg(test)]
	#[test]
	fn test_eval_number() {
		let mut interpreter = Interpreter::new(Program {
			body: vec![TokenValue::Number(1.0)],
		});
		let mut env = Environment::new(None);
		let result = interpreter.eval_program(&mut env);
		assert_eq!(result.get_type(), ValueType::Number);
		assert_eq!(
			result.as_any().downcast_ref::<NumberVal>().unwrap().value(),
			1.0
		);
	}

	#[test]
	fn test_eval_identifier() {
		let mut interpreter = Interpreter::new(Program {
			body: vec![TokenValue::Identifier("x".to_string())],
		});
		let mut env = Environment::new(None);
		env.define("x".to_string(), Rc::new(NumberVal::new(1.0)), false);
		let result = interpreter.eval_program(&mut env);
		assert_eq!(result.get_type(), ValueType::Number);
		assert_eq!(
			result.as_any().downcast_ref::<NumberVal>().unwrap().value(),
			1.0
		);
	}

	#[test]
	fn test_variable_decleration_with_number() {
		let mut interpreter = Interpreter::new(Program {
			body: vec![TokenValue::VarDeclaration(
				"x".to_string(),
				false,
				Rc::new(TokenValue::Number(1.0)),
			)],
		});
		let mut env = Environment::new(None);
		let result = interpreter.eval_program(&mut env);
		assert_eq!(result.get_type(), ValueType::Number);
		assert_eq!(
			result.as_any().downcast_ref::<NumberVal>().unwrap().value(),
			1.0
		);
	}

	#[test]
	fn test_variable_decleration_without_value() {
		let mut interpreter = Interpreter::new(Program {
			body: vec![TokenValue::VarDeclaration(
				"x".to_string(),
				false,
				Rc::new(TokenValue::Null),
			)],
		});
		let mut env = Environment::new(None);
		let result = interpreter.eval_program(&mut env);
		assert_eq!(result.get_type(), ValueType::Null);
	}

	




