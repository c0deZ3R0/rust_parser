use crate::lexer::tokens::{TokenType, TokenValue};
use logos::{Lexer, Span};
use std::rc::Rc;

use crate::parser::expressions::parse_expr;

use super::Parser;
// Other necessary imports...

type Error = (String, Span);
type ParseResult<T> = std::result::Result<T, Error>;

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
		_ => Err((
			"expected identifier name following let | const keywords".to_owned(),
			parser.lexer.span(),
		)),
	};

	parser.advance();

	match &parser.current_token {
		Some(Ok(TokenType::Semicolon)) => {
			if (is_const) {
				return Err((
					"Must assign value to const declaration.".to_owned(),
					parser.lexer.span(),
				));
			}
			Ok(TokenValue::VarDeclaration(
				identifier?,
				false,
				Rc::new(TokenValue::Null),
			))
		}
		Some(Ok(TokenType::Equals)) => {
			parser.advance(); // Advance to get the expression token
			let expr = parse_expr(parser)?; // Parse the expression
			match &parser.current_token {
				Some(Ok(TokenType::Semicolon)) => Ok(TokenValue::VarDeclaration(
					identifier?,
					is_const,
					Rc::new(expr),
				)),
				_ => Err((
					"expected semicolon after variable declaration".to_owned(),
					parser.lexer.span(),
				)),
			}
		}
		_ => Err((
			"expected equals sign after identifier name".to_owned(),
			parser.lexer.span(),
		)),
	}
}
