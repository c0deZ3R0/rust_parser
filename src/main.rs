use rust_parser::{runtime::{Interpreter, environment}, parser, Environment};





use std::io::{self, Write};


fn main() {

    let mut input = String::new();

    loop {
        print!("Enter code (or 'exit' to quit): ");
        io::stdout().flush().unwrap(); // Ensure the prompt is displayed before reading input

        input.clear(); // Clear the previous input
        io::stdin().read_line(&mut input).unwrap();

        let trimmed_input = input.trim();
        if trimmed_input.eq_ignore_ascii_case("exit") {
            break;
        }

        let mut parser = parser::Parser::new(trimmed_input);
        match parser.produce_ast() {
            Ok(ast) => {
                let mut interpreter = Interpreter::new(ast);
                let result = interpreter.eval_program(&mut Environment::new(None));
                println!("Result: {:#?}", result);
            }
            Err(e) => {
                println!("Error parsing input: {:?}", e);
            }
        }
    }
}