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
	let source_code = "const MY_CONST;";
	let mut parser = parser::Parser::new(source_code);
	let mut env = Environment::new(None);

	//env.define("x".to_string(), makenumber(110.00), false);
	//env.define("true".to_string(), makebool(Some(true)));
	//env.define("false".to_string(), makebool(Some(false)));

	let ast = parser.produce_ast();
	println!("{:#?}", ast);

	let mut interpreter = runtime::Interpreter::new(ast.unwrap());

	let result = interpreter.eval_program(&mut env);

	println!("{:#?}", result);
}
