pub mod parser;
mod runtime;
pub mod lexer;
use crate::runtime::values::{NumberVal, NullVal, RuntimeVal, ValueType};



fn main() {

let source_code = "100 * 10 + 1/2";
let mut parser = parser::Parser::new(source_code);
let ast = parser.produce_ast();

//println!("{:#?}", ast);



let mut interpreter = runtime::Interpreter::new(ast.unwrap());
let result = interpreter.eval_program();

let runtimevalue = result.as_any().downcast_ref::<NumberVal>().expect("Type mismatch");

println!("{:#?}", runtimevalue);




}