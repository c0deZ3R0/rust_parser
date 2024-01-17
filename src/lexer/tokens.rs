use logos::{Logos,Lexer, Span};
use std::collections::HashMap;
use std::rc::Rc;


#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\n\f]+")] // new way to annotate whitespace
pub enum TokenType {
    #[token("(")]
    OpenParen,
    #[token(")")]
    CloseParen,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Times,
    #[token("/")]
    Divide,
    #[token("=")]
    Equals,
    #[regex(r"(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?", |lex| lex.slice().parse::<f64>().unwrap())]
    Number(f64),

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Identifier(String),

    #[token("let")]
    Let,  

}


#[derive(Debug)]
pub enum TokenValue {
    /// null.
    Null,
    /// true or false.
    Bool(bool),
    /// Any floating point number.
    Number(f64),
    /// Any quoted string.
    String(String),
    /// An array of values
    Array(Vec<TokenValue>),
        /// An dictionary mapping keys and values.
    Object(HashMap<String, TokenValue>),

    BinaryExpr(Rc<TokenValue>, Rc<TokenValue>, TokenType),

    Identifier(String),

}