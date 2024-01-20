// src/errors.rs
use std::fmt;

use logos::Span;

// region:    --- Environment Error

pub enum EnvironmentError {
	VariableAlreadyDefined(String),
	VariableNotDefined(String),
	VariableCannotBeReassigned(String),
	ConstantAlreadyDefined(String),
	ConstantCannotBeReassigned(String),
}

impl fmt::Display for EnvironmentError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			EnvironmentError::VariableAlreadyDefined(name) => {
				write!(f, "Variable {} already defined", name)
			}
			EnvironmentError::VariableNotDefined(name) => {
				write!(f, "Variable {} not defined", name)
			}
			EnvironmentError::VariableCannotBeReassigned(name) => {
				write!(f, "Variable {} cannot be reassigned", name)
			}
			EnvironmentError::ConstantAlreadyDefined(name) => {
				write!(f, "Constant {} already defined", name)
			}
			EnvironmentError::ConstantCannotBeReassigned(name) => {
				write!(f, "Constant {} cannot be reassigned", name)
			}
		}
	}
}

// endregion: --- Environment Error


// region:    --- Parser Error

#[derive(Debug)]
pub enum ParserError {
	UnexpectedToken(String, Span),
	LexerError(String,Span),
	SyntaxError(String),
	InvalidToken(String),
	MissingIdentifier(Span),
	MissingEqualsSign(Span),
	MissingSemicolon(Span),
	ConstDeclarationMissingValue(Span),
	ConstLetMissingIdentifier(Span),
	PrimaryExprError(String)
	// Add more error types as needed
}

impl From<(String, std::ops::Range<usize>)> for ParserError {
	fn from((msg, range): (String, std::ops::Range<usize>)) -> Self {
		ParserError::SyntaxError(format!("{} at {:?}", msg, range))
	}
}



impl fmt::Display for ParserError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			
			ParserError::PrimaryExprError(msg) => write!(f, "Primary expression error: {}", msg),

			ParserError::UnexpectedToken(expected, span) => {
				write!(f, "Unexpected token {:?} at {:?}", expected, span)
			}
			ParserError::LexerError(msg, span) => write!(f, "Lexer error: {}", msg),
			ParserError::SyntaxError(msg) => write!(f, "Syntax error: {}", msg),
			ParserError::InvalidToken(msg) => write!(f, "Invalid token: {}", msg),
			ParserError::MissingIdentifier(span) => {
				write!(f, "Missing identifier at {:?}", span)
			}
			ParserError::MissingEqualsSign(span) => {
				write!(f, "Missing equals sign at {:?}", span)
			}
			ParserError::MissingSemicolon(span) => {
				write!(f, "Missing semicolon at {:?}", span)
			}
			ParserError::ConstDeclarationMissingValue(span) => {
				write!(f, "Const declaration missing value at {:?}", span)
			}
			ParserError::ConstLetMissingIdentifier(span) => {
				write!(f, "Const let missing identifier at {:?}", span)
			} // Handle other errors
		}
	}
}

impl std::error::Error for ParserError {}


// endregion: --- Parser Error