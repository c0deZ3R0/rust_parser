//src/main.rs

pub mod parser;
mod runtime;
pub mod lexer;
use crate::runtime::values::{NumberVal, NullVal, RuntimeValue, ValueType, makenumber, makebool};
use crate::runtime::environment::Environment;
use std::rc::Rc;


fn main() {

let source_code = "test + 10";
let mut parser = parser::Parser::new(source_code);
let mut env = Environment::new(None);

env.define("x".to_string(), makenumber(110.00));
env.define("test".to_string(), makebool(Some(true)));


let ast = parser.produce_ast();
println!("{:#?}", ast);



let mut interpreter = runtime::Interpreter::new(ast.unwrap());

let result = interpreter.eval_program(&env);



println!("{:#?}", result);




}