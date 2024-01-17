//src/main.rs

pub mod parser;
mod runtime;
pub mod lexer;
use crate::runtime::values::{NumberVal, NullVal, RuntimeValue, ValueType, makenumber, makebool};
use crate::runtime::environment::Environment;
use std::rc::Rc;


fn main() {

let source_code = "let y = 110;
let x = 50;
let z = 1000;
const CONST_TEST = 100;
const CONST_TEST = 101;
z + x + y + CONST_TEST";
let mut parser = parser::Parser::new(source_code);
let mut env = Environment::new(None);

//env.define("x".to_string(), makenumber(110.00), false);
//env.define("true".to_string(), makebool(Some(true)));
//env.define("false".to_string(), makebool(Some(false)));


let ast = parser.produce_ast();
println!("{:#?}", ast);



let mut interpreter = runtime::Interpreter::new(ast.unwrap());

let result = interpreter.eval_program(&mut env);



println!("{:#?}", result);




}