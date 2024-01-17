//src/main.rs

pub mod lexer;
pub mod parser;
mod runtime;
use crate::runtime::environment::Environment;
use crate::runtime::values::{
	makebool, makenumber, NullVal, NumberVal, RuntimeValue, ValueType,
};
use std::rc::Rc;

fn main() {
	
	let mut source_code = "const MY_CONST = 20; let x = 10; let y = 20; let z = x + y; z + MY_CONST; x = z; x";
	// Add a constant declaration
	
	
	let mut parser = parser::Parser::new(&source_code);
	let mut env = Environment::new(None);

	let ast = parser.produce_ast();
	let mut interpreter = runtime::Interpreter::new(ast.unwrap());

	let result = interpreter.eval_program(&mut env);

	println!("{:#?}", result);
}
