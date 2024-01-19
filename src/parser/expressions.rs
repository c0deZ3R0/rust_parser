use crate::tokens::{TokenType, TokenValue};
use logos::{Lexer, Span};
use std::rc::Rc;

use super::Parser;
// Other necessary imports...

use crate::ParserError;
type ParseResult<T> = Result<T, ParserError>;

pub fn parse_expr(parser: &mut Parser) -> ParseResult<TokenValue> {
	parse_additive_expr(parser)
}

pub fn parse_additive_expr(parser: &mut Parser) -> ParseResult<TokenValue> {
	let mut left = parse_multiplicative_expr(parser)?;
	while let Some(Ok(token)) = &parser.current_token {
		match token {
			TokenType::Plus | TokenType::Minus => {
				let operator = token.clone(); // Copy the token (cheap for simple enums)
				parser.advance();
				let right = parse_multiplicative_expr(parser)?;
				left =
					TokenValue::BinaryExpr(Rc::new(left), Rc::new(right), operator);
			}
			_ => break,
		}
	}
	Ok(left)
}

pub fn parse_multiplicative_expr(parser: &mut Parser) -> ParseResult<TokenValue> {
	let mut left = parse_primary_expr(parser)?;
	while let Some(Ok(ref token)) = parser.current_token {
		match *token {
			TokenType::Times | TokenType::Divide => {
				let operator = token.clone();
				parser.advance();
				let right = parse_primary_expr(parser)?;
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
