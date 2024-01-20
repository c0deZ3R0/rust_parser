use crate::tokens::{TokenType, TokenValue};
use logos::{Lexer, Span};
use std::rc::Rc;

use super::Parser;
// Other necessary imports...

use crate::ParserError;
type ParseResult<T> = Result<T, ParserError>;

pub fn parse_expr(parser: &mut Parser) -> ParseResult<TokenValue> {
	parse_assignment_expression(parser)
}

pub fn parse_assignment_expression(parser: &mut Parser) -> ParseResult<TokenValue> {
	let mut left = match parse_additive_expr(parser) {
		Ok(expr) => expr,
		Err(e) => return Err(ParserError::PrimaryExprError("Error in left operand of assignment".to_owned())),
	};

	while let Some(Ok(token)) = &parser.current_token {
		match token {
			TokenType::Equals => {
				parser.advance();
				let value = match parse_assignment_expression(parser) {
					Ok(expr) => expr,
					Err(e) => return Err(ParserError::PrimaryExprError("Error in right operand of assignment".to_owned())),
				};

				return(Ok(TokenValue::AssignmentExpr(Rc::new(left), Rc::new(value))));
			}
			_ => break,
		}
	}
	Ok(left)
}



pub fn parse_additive_expr(parser: &mut Parser) -> ParseResult<TokenValue> {
	let mut left = match parse_multiplicative_expr(parser) {
        Ok(expr) => expr,
        Err(e) => return Err(ParserError::PrimaryExprError("Error in left operand of addition/subtraction".to_owned())),
    };


	while let Some(Ok(token)) = &parser.current_token {
		match token {
			TokenType::Plus | TokenType::Minus => {
				let operator = token.clone(); // Copy the token (cheap for simple enums)
				parser.advance();
				let right = match parse_multiplicative_expr(parser) {
                    Ok(expr) => expr,
                    Err(e) => return Err(ParserError::PrimaryExprError("Error in right operand of addition/subtraction".to_owned())),
                };

				left =
					TokenValue::BinaryExpr(Rc::new(left), Rc::new(right), operator);
			}
			_ => break,
		}
	}
	Ok(left)
}

pub fn parse_multiplicative_expr(parser: &mut Parser) -> ParseResult<TokenValue> {
	let mut left = match parse_primary_expr(parser) {
        Ok(expr) => expr,
        Err(e) => return Err(ParserError::PrimaryExprError("Error in left operand of multiplication/division".to_owned())),
    };
	
	while let Some(Ok(ref token)) = parser.current_token {
		match *token {
			TokenType::Times | TokenType::Divide => {
				let operator = token.clone();
				parser.advance();
				let right = match parse_primary_expr(parser) {
                    Ok(expr) => expr,
                    Err(e) => return Err(ParserError::PrimaryExprError("Error in right operand of multiplication/division".to_owned())),
                };
				left =
					TokenValue::BinaryExpr(Rc::new(left), Rc::new(right), operator);
			}
			_ => break,
		}
	}
	Ok(left)
}

pub fn parse_primary_expr(parser: &mut Parser) -> ParseResult<TokenValue> {
	let current_token = parser.current_token.take();

	match current_token {
		Some(Ok(TokenType::Number(n))) => {
			parser.advance();
			Ok(TokenValue::Number(n))
		}
		Some(Ok(TokenType::Identifier(s))) => {
			parser.advance();
			Ok(TokenValue::Identifier(s))
		}
		_ => Err(ParserError::LexerError("parse_primary_expr".to_string(),parser.lexer.span())),
	}
}
