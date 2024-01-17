pub mod environment;
pub mod values;

use crate::lexer::tokens::TokenValue::{
	BinaryExpr, Identifier, Number, VarDeclaration,
};
use crate::lexer::tokens::{TokenType, TokenValue};
use crate::parser::{self, Program};
use std::rc::Rc;

use values::{NullVal, NumberVal, RuntimeVal, RuntimeValue, ValueType};

use self::environment::Environment;
use self::values::makenull;

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
			.map(|expr| self.eval(expr, env)) // Pass the immutable reference directly
			.last()
			.unwrap_or_else(|| Rc::new(NullVal))
	}

	fn eval(
		&self,
		expr: &TokenValue,
		env: &mut Environment,
	) -> Rc<dyn RuntimeValue> {
		match expr {
			TokenValue::Number(n) => Rc::new(NumberVal::new(*n)),
			TokenValue::Identifier(name) => self.iden(name, env),
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
				self.vardec(name, *is_const, Some(expr), env)
			}

			_ => unimplemented!(),
		}
	}

	fn vardec(
		&self,
		name: &str,
		is_const: bool,
		expr: Option<&TokenValue>,
		env: &mut Environment,
	) -> Rc<dyn RuntimeValue> {
		let value = match expr {
			Some(expr_rc) => self.eval(expr_rc, env),
			None => makenull(),
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
