
pub mod values;


use crate::lexer::tokens::{TokenValue, TokenType};
use crate::parser::{self, Program};

use values::{NumberVal, NullVal, RuntimeVal, ValueType};

pub struct Interpreter {
    ast:Program,
}

impl Interpreter {
    pub fn new(ast:Program) -> Self {
        Self {
            ast,
        }
    }
  
   pub fn eval_program(&mut self) -> Box<dyn RuntimeVal> {
        self.ast.body.iter().map(|expr| self.eval(expr)).last().unwrap_or_else(|| Box::new(NullVal))
    }

    fn eval(&self, expr: &TokenValue) -> Box<dyn RuntimeVal> {
        match expr {
            TokenValue::Number(n) => Box::new(NumberVal::new(*n)),
            TokenValue::BinaryExpr(left, right, op) => {
                let left_val = self.eval(left);
                let right_val = self.eval(right);

                match op {
                    TokenType::Plus => self.add(left_val, right_val),
                    TokenType::Minus => self.sub(left_val, right_val),
                    TokenType::Times => self.mul(left_val, right_val),
                    TokenType::Divide => self.div(left_val, right_val),
                    // Handle other operators
                    _ => unimplemented!(),
                }
            },
            _ => unimplemented!(),
        }
    }

    fn sub(&self, left: Box<dyn RuntimeVal>, right: Box<dyn RuntimeVal>) -> Box<dyn RuntimeVal> {

        match (left.as_ref(), right.as_ref()) {
            (left_val, right_val) => {
                // Check if both values are NumberVal
                if left_val.get_type() == ValueType::Number && right_val.get_type() == ValueType::Number {
                    // Perform addition
                    let left_number = left_val.as_any().downcast_ref::<NumberVal>().expect("Type mismatch");
                    let right_number = right_val.as_any().downcast_ref::<NumberVal>().expect("Type mismatch");
                    Box::new(NumberVal::new(left_number.value() - right_number.value()))
                } else {
                    // Handle other types or errors
                    unimplemented!()
                }
            }
        }
    }

    fn add(&self, left: Box<dyn RuntimeVal>, right: Box<dyn RuntimeVal>) -> Box<dyn RuntimeVal> {
        match (left.as_ref(), right.as_ref()) {
            (left_val, right_val) => {
                // Check if both values are NumberVal
                if left_val.get_type() == ValueType::Number && right_val.get_type() == ValueType::Number {
                    // Perform addition
                    let left_number = left_val.as_any().downcast_ref::<NumberVal>().expect("Type mismatch");
                    let right_number = right_val.as_any().downcast_ref::<NumberVal>().expect("Type mismatch");
                    Box::new(NumberVal::new(left_number.value() + right_number.value()))
                } else {
                    // Handle other types or errors
                    unimplemented!()
                }
            }
        }
    }

    fn mul(&self, left: Box<dyn RuntimeVal>, right: Box<dyn RuntimeVal>) -> Box<dyn RuntimeVal> {
        match (left.as_ref(), right.as_ref()) {
            (left_val, right_val) => {
                // Check if both values are NumberVal
                if left_val.get_type() == ValueType::Number && right_val.get_type() == ValueType::Number {
                    // Perform addition
                    let left_number = left_val.as_any().downcast_ref::<NumberVal>().expect("Type mismatch");
                    let right_number = right_val.as_any().downcast_ref::<NumberVal>().expect("Type mismatch");
                    Box::new(NumberVal::new(left_number.value() * right_number.value()))
                } else {
                    // Handle other types or errors
                    unimplemented!()
                }
            }
        }
    }

    fn div(&self, left: Box<dyn RuntimeVal>, right: Box<dyn RuntimeVal>) -> Box<dyn RuntimeVal> {
        match (left.as_ref(), right.as_ref()) {
            (left_val, right_val) => {
                // Check if both values are NumberVal
                if left_val.get_type() == ValueType::Number && right_val.get_type() == ValueType::Number {
                    // Perform addition
                    let left_number = left_val.as_any().downcast_ref::<NumberVal>().expect("Type mismatch");
                    let right_number = right_val.as_any().downcast_ref::<NumberVal>().expect("Type mismatch");
                    Box::new(NumberVal::new(left_number.value() / right_number.value()))
                } else {
                    // Handle other types or errors
                    unimplemented!()
                }
            }
        }
    }
}
    



    



