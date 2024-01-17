
pub mod values;
pub mod environment;

use std::rc::Rc;
use crate::lexer::tokens::{TokenValue, TokenType};
use crate::parser::{self, Program};

use tokio::runtime::Runtime;
use values::{NumberVal, NullVal,RuntimeVal, RuntimeValue, ValueType};

use self::environment::Environment;

pub struct Interpreter {
    ast:Program,
    env:Environment,
}

impl Interpreter {
    pub fn new(ast:Program) -> Self {
        Self {
            ast,
            env:Environment::new(None),
        }
    }
  
   pub fn eval_program(&mut self, env:&Environment) -> Rc<dyn RuntimeValue> {
    self.env = env.clone();

    self.ast.body.iter()
    .map(|expr| self.eval(expr, &self.env))
    .last()
    .unwrap_or_else(|| Rc::new(NullVal))
    }

    fn eval(&self, expr: &TokenValue,  env:&Environment) -> Rc<dyn RuntimeValue> {
        match expr {
            TokenValue::Number(n) => Rc::new(NumberVal::new(*n)),
            TokenValue::Identifier(name) => self.iden(name, env),
            TokenValue::BinaryExpr(left, right, op) => {
                let left_val = self.eval(left, env);
                let right_val = self.eval(right, env);

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


    fn iden(&self, iden: &String, env: &Environment) -> Rc<dyn RuntimeValue> {
        match env.lookup(iden) {
            Some(val) => val.clone(),
            None => Rc::new(NullVal),
        }
    }

    fn sub(&self, left: Rc<dyn RuntimeValue>, right: Rc<dyn RuntimeValue>) -> Rc<dyn RuntimeValue> {

        match (left.as_ref(), right.as_ref()) {
            (left_val, right_val) => {
                // Check if both values are NumberVal
                if left_val.get_type() == ValueType::Number && right_val.get_type() == ValueType::Number {
                    // Perform addition
                    let left_number = left_val.as_any().downcast_ref::<NumberVal>().expect("Type mismatch");
                    let right_number = right_val.as_any().downcast_ref::<NumberVal>().expect("Type mismatch");
                    Rc::new(NumberVal::new(left_number.value() - right_number.value()))
                } else {
                    // Handle other types or errors
                    unimplemented!()
                }
            }
        }
    }

    fn add(&self, left: Rc<dyn RuntimeValue>, right: Rc<dyn RuntimeValue>) -> Rc<dyn RuntimeValue> {
        match (left.as_ref(), right.as_ref()) {
            (left_val, right_val) => {
                // Check if both values are NumberVal
                if left_val.get_type() == ValueType::Number && right_val.get_type() == ValueType::Number {
                    // Perform addition
                    let left_number = left_val.as_any().downcast_ref::<NumberVal>().expect("Type mismatch");
                    let right_number = right_val.as_any().downcast_ref::<NumberVal>().expect("Type mismatch");
                    Rc::new(NumberVal::new(left_number.value() + right_number.value()))
                } else {
                    // Handle other types or errors
                    unimplemented!()
                }
            }
        }
    }

    fn mul(&self, left: Rc<dyn RuntimeValue>, right: Rc<dyn RuntimeValue>) -> Rc<dyn RuntimeValue> {
        match (left.as_ref(), right.as_ref()) {
            (left_val, right_val) => {
                // Check if both values are NumberVal
                if left_val.get_type() == ValueType::Number && right_val.get_type() == ValueType::Number {
                    // Perform addition
                    let left_number = left_val.as_any().downcast_ref::<NumberVal>().expect("Type mismatch");
                    let right_number = right_val.as_any().downcast_ref::<NumberVal>().expect("Type mismatch");
                    Rc::new(NumberVal::new(left_number.value() * right_number.value()))
                } else {
                    // Handle other types or errors
                    unimplemented!()
                }
            }
        }
    }

    fn div(&self, left: Rc<dyn RuntimeValue>, right: Rc<dyn RuntimeValue>) -> Rc<dyn RuntimeValue> {
        match (left.as_ref(), right.as_ref()) {
            (left_val, right_val) => {
                // Check if both values are NumberVal
                if left_val.get_type() == ValueType::Number && right_val.get_type() == ValueType::Number {
                    // Perform addition
                    let left_number = left_val.as_any().downcast_ref::<NumberVal>().expect("Type mismatch");
                    let right_number = right_val.as_any().downcast_ref::<NumberVal>().expect("Type mismatch");
                    Rc::new(NumberVal::new(left_number.value() / right_number.value()))
                } else {
                    // Handle other types or errors
                    unimplemented!()
                }
            }
        }
    }
}
    



    



