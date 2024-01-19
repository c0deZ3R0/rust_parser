use logos::{Lexer, Span};
use std::rc::Rc;

use crate::parser::expressions::parse_expr;
use crate::tokens::*;

use crate::ParserError;
type ParseResult<T> = Result<T, ParserError>;

use super::Parser;

pub fn parse_stmt(parser: &mut Parser) -> ParseResult<TokenValue> {
	match parser.current_token {
		Some(Ok(TokenType::Let)) | Some(Ok(TokenType::Const)) => {
			parse_vardec_stmt(parser)
		}
		// Assuming parse_expr also returns ParseResult<TokenValue>
		_ => parse_expr(parser),
	}
}


pub fn parse_vardec_stmt(parser: &mut Parser) -> ParseResult<TokenValue> {
	let is_const = match parser.current_token.take() {
		Some(Ok(TokenType::Const)) => true,
		_ => false,
	};
	parser.advance(); // Advance to get the identifier token

	let identifier = match &parser.current_token.take() {
		Some(Ok(TokenType::Identifier(s))) => {
			// Clone the string here to avoid moving out of borrowed context
			let identifier = s.clone();
			Ok(identifier)
		}
		_ => Err(ParserError::ConstLetMissingIdentifier(parser.lexer.span())),
	};

	parser.advance();

	match &parser.current_token {
		Some(Ok(TokenType::Semicolon)) => {
			if (is_const) {
				return Err(ParserError::ConstDeclarationMissingValue(parser.lexer.span()));
			}
			Ok(TokenValue::VarDeclaration(
				identifier?,
				false,
				Rc::new(TokenValue::Null),
			))
		}
		Some(Ok(TokenType::Equals)) => {
			parser.advance(); 
			let expr = parse_expr(parser)?;
			match &parser.current_token {
				Some(Ok(TokenType::Semicolon)) => Ok(TokenValue::VarDeclaration(
					identifier?,
					is_const,
					Rc::new(expr),
				)),
				_ => Err(((ParserError::MissingSemicolon(parser.lexer.span()))
				)),
			}
		}
		_ => Err(((ParserError::MissingEqualsSign(parser.lexer.span()))
		)),
	}
}