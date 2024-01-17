// Module: parser
// Path: src/parser/mod.rs


use std::{env, string};
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
            self.current_token = self.lexer.next();
        }

        fn next_token(&mut self) -> Option<Result<TokenType, ()>> {
            let token = self.current_token.take(); // Take the current token
            self.current_token = self.lexer.next(); // Advance to the next token
            token
        }
    
    
        fn parse_stmt(&mut self) -> ParseResult<TokenValue> {
            match self.current_token {
                Some(Ok(TokenType::Let)) | Some(Ok(TokenType::Const)) => {
                    self.parse_vardec_stmt()
                },
                // Assuming parse_expr also returns ParseResult<TokenValue>
                _ => self.parse_expr(),
            }
        }

    fn parse_vardec_stmt(&mut self) ->  ParseResult<TokenValue>{
        
        let is_const =  match self.current_token.take() {
            Some(Ok(TokenType::Const)) => true,
            _ => false,
        };
        self.advance(); // Advance to get the identifier token
        
        let identifier = match &self.current_token.take() {
            Some(Ok(TokenType::Identifier(s))) => {
                // Clone the string here to avoid moving out of borrowed context
                let identifier = s.clone();
                Ok(identifier)
            },
            _ => Err(("expected identifier name following let | const keywords".to_owned(), self.lexer.span())),
        };
        
        
        self.advance(); 
        
        match &self.current_token {
            Some(Ok(TokenType::Semicolon)) => {
                if(is_const){
                 return Err(("Must assign value to const declaration.".to_owned(), self.lexer.span()))
                }         
                Ok(TokenValue::VarDeclaration(identifier?, false, Rc::new(TokenValue::Null)))
            },
            Some(Ok(TokenType::Equals)) => {
                self.advance(); // Advance to get the expression token
                let expr = self.parse_expr()?; // Parse the expression
                match &self.current_token {
                    Some(Ok(TokenType::Semicolon)) => {
                        Ok(TokenValue::VarDeclaration(identifier?, is_const, Rc::new(expr)))
                    },
                    _ => Err(("expected semicolon after variable declaration".to_owned(), self.lexer.span())),
                }
            },
            _ => Err(("expected equals sign after identifier name".to_owned(), self.lexer.span())),
        }

    }
    
  

    
    fn parse_expr(&mut self) -> ParseResult<TokenValue> {
     
        self.parse_additive_expr()
    }

    fn parse_additive_expr(&mut self) -> ParseResult<TokenValue> {
        let mut left = self.parse_multiplicative_expr()?;
        while let Some(Ok(token)) = &self.current_token {
            match token {
                TokenType::Plus | TokenType::Minus => {
                    let operator = token.clone(); // Copy the token (cheap for simple enums)
                    self.advance();
                    let right = self.parse_multiplicative_expr()?;
                    left = TokenValue::BinaryExpr(Rc::new(left), Rc::new(right), operator);
                },
                _ => break,
            }
        }
        Ok(left)
    }
    
    fn parse_multiplicative_expr(&mut self) -> ParseResult<TokenValue> {
        let mut left = self.parse_primary_expr()?;
        while let Some(Ok(ref token)) = self.current_token {
            match *token {
                TokenType::Times | TokenType::Divide => {
                    let operator = token.clone();
                    self.advance();
                    let right = self.parse_primary_expr()?;
                    left = TokenValue::BinaryExpr(Rc::new(left), Rc::new(right), operator);
                },
                _ => break,
            }
        }
        Ok(left)
    }
    
    fn parse_primary_expr(&mut self) -> ParseResult<TokenValue> {
       let current_token = self.current_token.take();
    
        match current_token {
            Some(Ok(TokenType::Number(n))) => {
                self.advance();
                Ok(TokenValue::Number(n)) 
            },
            Some(Ok(TokenType::Identifier(s))) => {
                self.advance();
                Ok(TokenValue::Identifier(s)) 
            },
            _ => {
                self.current_token = current_token;
                Err(("unexpected token in primary expression".to_owned(), self.lexer.span()))
            }
        }
    }
}






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
        },
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
        },
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
        },
        _ => panic!("Expected a variable declaration"),
    }
}


#[test]
fn test_const_declaration_without_value_should_fail() {
    let source_code = "const myConst;";
    let mut parser = Parser::new(source_code);
    
    // We expect the parser to return an error
    match parser.produce_ast() {
        Ok(_) => panic!("Parser should fail on const declaration without an initial value"),
        Err((msg, _)) => {
            // Check if the error message is what we expect
            assert!(msg.contains("Must assign value to const declaration."), "Unexpected error message: {}", msg);
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
        },
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
        },
        _ => panic!("Expected a binary subtraction expression"),
    }
}

#[test]
fn test_parse_multiplication() {
    let source_code = "6 * 2";
    let mut parser = Parser::new(source_code);
    let ast = parser.produce_ast().expect("Failed to parse multiplication");

    assert_eq!(ast.body.len(), 1);
    match &ast.body[0] {
        TokenValue::BinaryExpr(left, right, op) => {
            assert_eq!(**left, TokenValue::Number(6.0));
            assert_eq!(**right, TokenValue::Number(2.0));
            assert_eq!(*op, TokenType::Times);
        },
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
            },
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
                            },
                            _ => panic!("Expected a multiplication expression"),
                        }
                    },
                    _ => panic!("Expected an addition expression"),
                }

                // Right side of the top-level operation: 5 / 2
                match &**right {
                    TokenValue::BinaryExpr(rl, rr, rop) => {
                        assert_eq!(**rl, TokenValue::Number(5.0));
                        assert_eq!(**rr, TokenValue::Number(2.0));
                        assert_eq!(*rop, TokenType::Divide);
                    },
                    _ => panic!("Expected a division expression"),
                }
            },
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
        },
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
        },
        _ => panic!("Expected a constant declaration as the second element"),
    }

    // Check the binary expression
    match &ast.body[2] {
        TokenValue::BinaryExpr(left, right, op) => {
            match (&**left, &**right) {
                (TokenValue::Identifier(lname), TokenValue::Identifier(rname)) => {
                    assert_eq!(lname, "var1");
                    assert_eq!(rname, "var2");
                },
                _ => panic!("Expected identifiers in the binary expression"),
            }
            assert_eq!(*op, TokenType::Plus);
        },
        _ => panic!("Expected a binary expression as the third element"),
    }
}
    // Additional tests...
