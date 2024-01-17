//src/main.rs

pub mod parser;
mod runtime;
pub mod lexer;
use crate::runtime::values::{NumberVal, NullVal, RuntimeValue, ValueType};
use crate::runtime::environment::Environment;
use std::rc::Rc;


fn main() {

let source_code = "x + 100000";
let mut parser = parser::Parser::new(source_code);
let mut env = Environment::new(None);

env.define("x".to_string(), Rc::new(NumberVal::new(10.0)));


let ast = parser.produce_ast();
println!("{:#?}", ast);



let mut interpreter = runtime::Interpreter::new(ast.unwrap());

let result = interpreter.eval_program(&env);



println!("{:#?}", result);




}