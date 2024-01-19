// region:    --- Modules

pub mod lexer;
pub mod parser;
pub mod runtime;
pub use crate::runtime::environment::Environment;
pub use crate::runtime::values::{
	makebool, makenumber, NullVal, NumberVal, RuntimeValue, ValueType,
};

pub type Error = Box<dyn std::error::Error>;

// endregion: --- Modules

#[cfg(test)]
mod tests {
	use self::runtime::Interpreter;

use super::*;

	#[test]
	fn test_parser() {
		let source_code = "let x = 5; let y = 10; let z = x + y;";
		let mut interpreter = match parser::Parser::new(&source_code).produce_ast() {
			Ok(ast) => Interpreter::new(ast),
			Err(e) => {
				// Handle the error here, e.g., log it or convert it into a different format
				panic!("Parser error: {:?}", e);
			}
		};
	}
}
	

