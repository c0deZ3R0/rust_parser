// Module: parser
// Path: src/parser/mod.rs

// region:    --- Modules
mod expressions;
mod statements;

// endregion: --- Modules

// region:    --- Imports
use crate::ParserError;

use logos::{Lexer, Logos, Span};

use crate::tokens::*;
use expressions::*;
use statements::*;
use std::rc::Rc;

// endregion: --- Imports

use crate::{
	runtime::values::{makenull, NullVal},
	tokens::{TokenType, TokenValue},
};

type ParseResult<T> = Result<T, ParserError>;

#[derive(Debug)]
pub struct Program {
	pub body: Vec<TokenValue>,
}

pub struct Parser<'a> {
	lexer: Lexer<'a, TokenType>,
	current_token: Option<Result<TokenType, ()>>,
}

impl<'a> Parser<'a> {
	// Constructor for the Parser
	pub fn new(source_code: &'a str) -> Self {
		let mut lexer = TokenType::lexer(source_code);
		let current_token = lexer.next();

		Self {
			lexer,
			current_token,
		}
	}

	pub fn produce_ast(mut self) -> ParseResult<Program> {
		let mut program = Program { body: Vec::new() };
	
		while let Some(token_result) = self.current_token.clone() {
			match token_result {
				Ok(_) => match parse_stmt(&mut self) {
					Ok(value) => program.body.push(value),
					Err(e) => return Err(e),
				},
				Err(_) => {
					return Err(ParserError::LexerError(
						"produce_ast".to_string(),
						self.lexer.span(),
					));
				}
			}
			self.advance();
		}
	
		Ok(program)
	}
	fn advance(&mut self) {
		self.current_token = self.lexer.next();
	}

	fn next_token(&mut self) -> Option<Result<TokenType, ()>> {
		let token = self.current_token.take(); 
		self.current_token = self.lexer.next(); 
		token
	}
}

// region:    --- Tests

#[cfg(test)]
use super::*;

#[test]
fn test_parse_simple_number() {
	let source_code = "42";
	let mut parser = Parser::new(source_code);
	let ast = parser.produce_ast().expect("Failed to parse");

	assert_eq!(ast.body.len(), 1);
	match ast.body[0] {
		TokenValue::Number(n) => assert_eq!(n, 42.0),
		_ => panic!("Expected a number"),
	}
}

#[test]
fn test_parse_float_number() {
	let source_code = "3.14";
	let mut parser = Parser::new(source_code);
	let ast = parser.produce_ast().expect("Failed to parse");

	assert_eq!(ast.body.len(), 1);
	match ast.body[0] {
		TokenValue::Number(n) => assert_eq!(n, 3.14),
		_ => panic!("Expected a number"),
	}
}

#[test]
fn test_parse_simple_identifier() {
	let source_code = "myVariable";
	let mut parser = Parser::new(source_code);
	let ast = parser.produce_ast().expect("Failed to parse");

	assert_eq!(ast.body.len(), 1);
	match ast.body[0] {
		TokenValue::Identifier(ref name) => assert_eq!(name, "myVariable"),
		_ => panic!("Expected an identifier"),
	}
}

#[test]
fn test_parse_const_declaration() {
	let source_code = "const myConst = 10;";
	let mut parser = Parser::new(source_code);
	let ast = parser.produce_ast().expect("Failed to parse");

	assert_eq!(ast.body.len(), 1);
	match &ast.body[0] {
		TokenValue::VarDeclaration(name, is_const, expr) => {
			assert_eq!(name, "myConst");
			assert!(*is_const);
			match **expr {
				TokenValue::Number(n) => assert_eq!(n, 10.0),
				_ => panic!("Expected a number in const declaration"),
			}
		}
		_ => panic!("Expected a variable declaration"),
	}
}

#[test]
fn test_parse_var_declaration_with_initial_value() {
	let source_code = "let myVar = 42;";
	let mut parser = Parser::new(source_code);
	let ast = parser.produce_ast().expect("Failed to parse");

	assert_eq!(ast.body.len(), 1);
	match &ast.body[0] {
		TokenValue::VarDeclaration(name, is_const, expr) => {
			assert_eq!(name, "myVar");
			assert!(!is_const);
			match **expr {
				TokenValue::Number(n) => assert_eq!(n, 42.0),
				_ => panic!("Expected a number in variable declaration"),
			}
		}
		_ => panic!("Expected a variable declaration"),
	}
}

#[test]
fn test_parse_var_declaration_without_initial_value() {
	let source_code = "let myVar;";
	let mut parser = Parser::new(source_code);
	let ast = parser.produce_ast().expect("Failed to parse");

	assert_eq!(ast.body.len(), 1);
	match &ast.body[0] {
		TokenValue::VarDeclaration(name, is_const, expr) => {
			assert_eq!(name, "myVar");
			assert!(!is_const);
			assert!(matches!(**expr, TokenValue::Null));
		}
		_ => panic!("Expected a variable declaration"),
	}
}

#[test]
fn test_const_declaration_without_value_should_fail() {
	let source_code = "const myConst;";
	let mut parser = Parser::new(source_code);

	// We expect the parser to return an error
	match parser.produce_ast() {
		Ok(_) => panic!(
			"Parser should fail on const declaration without an initial value"
		),
		Err(e) => match e {
			ParserError::ConstDeclarationMissingValue(_) => {} // Test passes
			_ => panic!("Unexpected error type: {:?}", e),
		},
	}
}

#[test]
fn test_parse_addition() {
	let source_code = "3 + 7";
	let mut parser = Parser::new(source_code);
	let ast = parser.produce_ast().expect("Failed to parse addition");

	assert_eq!(ast.body.len(), 1);
	match &ast.body[0] {
		TokenValue::BinaryExpr(left, right, op) => {
			assert_eq!(**left, TokenValue::Number(3.0));
			assert_eq!(**right, TokenValue::Number(7.0));
			assert_eq!(*op, TokenType::Plus);
		}
		_ => panic!("Expected a binary addition expression"),
	}
}

#[test]
fn test_parse_subtraction() {
	let source_code = "10 - 4";
	let mut parser = Parser::new(source_code);
	let ast = parser.produce_ast().expect("Failed to parse subtraction");

	assert_eq!(ast.body.len(), 1);
	match &ast.body[0] {
		TokenValue::BinaryExpr(left, right, op) => {
			assert_eq!(**left, TokenValue::Number(10.0));
			assert_eq!(**right, TokenValue::Number(4.0));
			assert_eq!(*op, TokenType::Minus);
		}
		_ => panic!("Expected a binary subtraction expression"),
	}
}

#[test]
fn test_parse_multiplication() {
	let source_code = "6 * 2";
	let mut parser = Parser::new(source_code);
	let ast = parser
		.produce_ast()
		.expect("Failed to parse multiplication");

	assert_eq!(ast.body.len(), 1);
	match &ast.body[0] {
		TokenValue::BinaryExpr(left, right, op) => {
			assert_eq!(**left, TokenValue::Number(6.0));
			assert_eq!(**right, TokenValue::Number(2.0));
			assert_eq!(*op, TokenType::Times);
		}
		_ => panic!("Expected a binary multiplication expression"),
	}
}

#[test]
fn test_parse_division() {
	let source_code = "20 / 5";
	let mut parser = Parser::new(source_code);
	let ast = parser.produce_ast().expect("Failed to parse division");

	assert_eq!(ast.body.len(), 1);
	match &ast.body[0] {
		TokenValue::BinaryExpr(left, right, op) => {
			assert_eq!(**left, TokenValue::Number(20.0));
			assert_eq!(**right, TokenValue::Number(5.0));
			assert_eq!(*op, TokenType::Divide);
		}
		_ => panic!("Expected a binary division expression"),
	}
}

#[test]
fn test_parse_precedence() {
	// Test expression: 2 + 3 * 4 - 5 / 2
	// Expected parsing: 2 + ((3 * 4) - (5 / 2))
	let source_code = "2 + 3 * 4 - 5 / 2";
	let mut parser = Parser::new(source_code);
	let ast = parser.produce_ast().expect("Failed to parse precedence");

	assert_eq!(ast.body.len(), 1);
	match &ast.body[0] {
		TokenValue::BinaryExpr(left, right, op) => {
			// Check the top-level operation: -
			assert_eq!(*op, TokenType::Minus);

			// Left side of the top-level operation: 2 + (3 * 4)
			match &**left {
				TokenValue::BinaryExpr(ll, lr, lop) => {
					assert_eq!(**ll, TokenValue::Number(2.0));
					assert_eq!(*lop, TokenType::Plus);
					// Right side of the addition: 3 * 4
					match &**lr {
						TokenValue::BinaryExpr(lrl, lrr, lrop) => {
							assert_eq!(**lrl, TokenValue::Number(3.0));
							assert_eq!(**lrr, TokenValue::Number(4.0));
							assert_eq!(*lrop, TokenType::Times);
						}
						_ => panic!("Expected a multiplication expression"),
					}
				}
				_ => panic!("Expected an addition expression"),
			}

			// Right side of the top-level operation: 5 / 2
			match &**right {
				TokenValue::BinaryExpr(rl, rr, rop) => {
					assert_eq!(**rl, TokenValue::Number(5.0));
					assert_eq!(**rr, TokenValue::Number(2.0));
					assert_eq!(*rop, TokenType::Divide);
				}
				_ => panic!("Expected a division expression"),
			}
		}
		_ => panic!("Expected a binary expression"),
	}
}

#[test]
fn test_whitespace_independence() {
	let source_code = "
        let    var1 = 5;
        const  var2   =   10 ;
        var1 +     var2
    ";
	let mut parser = Parser::new(source_code);
	let ast = parser.produce_ast().expect("Failed to parse");

	// Check if the AST contains the expected elements
	assert_eq!(ast.body.len(), 3);

	// Check the first declaration
	match &ast.body[0] {
		TokenValue::VarDeclaration(name, is_const, expr) => {
			assert_eq!(name, "var1");
			assert!(!is_const);
			match **expr {
				TokenValue::Number(n) => assert_eq!(n, 5.0),
				_ => panic!("Expected a number in the first declaration"),
			}
		}
		_ => panic!("Expected a variable declaration as the first element"),
	}

	// Check the second declaration
	match &ast.body[1] {
		TokenValue::VarDeclaration(name, is_const, expr) => {
			assert_eq!(name, "var2");
			assert!(*is_const);
			match **expr {
				TokenValue::Number(n) => assert_eq!(n, 10.0),
				_ => panic!("Expected a number in the second declaration"),
			}
		}
		_ => panic!("Expected a constant declaration as the second element"),
	}

	// Check the binary expression
	match &ast.body[2] {
		TokenValue::BinaryExpr(left, right, op) => {
			match (&**left, &**right) {
				(TokenValue::Identifier(lname), TokenValue::Identifier(rname)) => {
					assert_eq!(lname, "var1");
					assert_eq!(rname, "var2");
				}
				_ => panic!("Expected identifiers in the binary expression"),
			}
			assert_eq!(*op, TokenType::Plus);
		}
		_ => panic!("Expected a binary expression as the third element"),
	}
}

#[test]
fn test_boolean_literal() {
	let source_code = "true";
	let mut parser = Parser::new(source_code);
	let ast = parser.produce_ast().expect("Failed to parse boolean literal");

	assert_eq!(ast.body.len(), 1);
	match &ast.body[0] {
		TokenValue::Identifier(b) => assert_eq!(b, "true"),
		_ => panic!("Expected a boolean literal"),
	}
}

// endregion: --- Tests
