// Module: parser
// Path: src/parser/mod.rs
use std::env;
use std::rc::Rc;
use logos::{Logos,Lexer,Span};

use crate::{lexer::tokens::{TokenValue, TokenType}, runtime::values::{NullVal, makenull}};


 
type Error = (String, Span);

type ParseResult<T> = std::result::Result<T, Error>;

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
            lexer, current_token 
          
        }
    }

    pub fn produce_ast(mut self) -> ParseResult<Program> {
        
   
        let mut program = Program {
            body: Vec::new(),
        };

        while let Some(token_result) = self.current_token.clone() {
            match token_result {
                Ok(token) => {
                    
                    match self.parse_stmt() {
                        Ok(value) => program.body.push(value),
                        Err((msg, span)) => return Err((msg, span)),
                    }
                },
                Err(_) => return Err(("Lexer error".to_string(), self.lexer.span())),
            }
            self.advance(); // Advance to the next token after processing
        }
    
        Ok(program)
            
        }

        fn advance(&mut self) {
            println!("{:#?}", self.current_token);
            self.current_token = self.lexer.next();
        }
    
    
        fn parse_stmt(&mut self) -> ParseResult<TokenValue> {
            match self.current_token.clone() {
                Some(Ok(TokenType::Let)) | Some(Ok(TokenType::Const)) => {
                    self.parse_vardec_stmt()
                },
                // Assuming parse_expr also returns ParseResult<TokenValue>
                _ => self.parse_expr(),
            }
        }

    fn parse_vardec_stmt(&mut self) ->  ParseResult<TokenValue>{
        let mut is_const = self.current_token.clone().unwrap().unwrap() == TokenType::Const;
        self.advance(); // Advance to get the identifier token
        let identifier = match self.current_token.clone() {
            Some(Ok(TokenType::Identifier(s))) => s,
            _ => return Err(("expected identifier name following let | const keywords".to_owned(), self.lexer.span())),
        };
        self.advance(); // Advance to get the equals sign token
        
        match self.current_token.clone() {
            Some(Ok(TokenType::Semicolon)) => {
                if(is_const){
                 return Err(("Must assign value to const declaration.".to_owned(), self.lexer.span()))
                }         
                Ok(TokenValue::VarDeclaration(identifier, is_const, Rc::new(TokenValue::Null)))
            },
            Some(Ok(TokenType::Equals)) => {
                self.advance(); // Advance to get the expression token
                let expr = self.parse_expr()?; // Parse the expression
                match self.current_token.clone() {
                    Some(Ok(TokenType::Semicolon)) => {
                        Ok(TokenValue::VarDeclaration(identifier, is_const, Rc::new(expr)))
                    },
                    _ => Err(("expected semicolon after variable declaration".to_owned(), self.lexer.span())),
                }
            },
            _ => Err(("expected equals sign after identifier name".to_owned(), self.lexer.span())),
        }

    }
    fn parse_expr(&mut self) -> ParseResult<TokenValue> {
        // Your parsing logic here
        // For example:
       
        self.parse_additive_expr()
    }

    fn parse_additive_expr(&mut self) -> ParseResult<TokenValue> {

        let mut left = self.parse_multiplicative_expr()?; // Parse the left-hand side
    
        while let Some(Ok(ref token)) = self.current_token {
            match token {
                    TokenType::Plus | TokenType::Minus => {
                   
                    let operator = token.clone();
                   
                    self.advance(); // Advance to get the right-hand side token
                    let right = self.parse_multiplicative_expr()?; // Parse the right-hand side
                    left = TokenValue::BinaryExpr(Rc::new(left), Rc::new(right), operator.clone());
                }
                _ => break,
            }
        }
    
        Ok(left)
    }
    
    fn parse_multiplicative_expr(&mut self) -> ParseResult<TokenValue> {
        
        let mut left = self.parse_primary_expr()?; // Parse the left-hand side
    
        while let Some(Ok(ref token)) = self.current_token {
            match token {
                TokenType::Times | TokenType::Divide => {
                    let operator = token.clone();
                    self.advance(); // Advance to get the right-hand side token
                    let right = self.parse_primary_expr()?; // Parse the right-hand side
                    left = TokenValue::BinaryExpr(Rc::new(left), Rc::new(right), operator.clone());
                }
                _ => break,
            }
        }
    
        Ok(left)
    }
    
    fn parse_primary_expr(&mut self) -> ParseResult<TokenValue> {
   
        match self.current_token.clone() {
            Some(Ok(TokenType::Number(n))) => {
                self.advance(); // Advance after parsing a number
                Ok(TokenValue::Number(n))
            }
            Some(Ok(TokenType::Identifier(s))) => {
                
                self.advance(); // Advance after parsing an identifier
                Ok(TokenValue::Identifier(s.clone()))
            }
            _ => Err(("unexpected token here (context: primary expression)".to_owned(), self.lexer.span())),
        }
    }
}