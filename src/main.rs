mod ast;
mod interpreter;
mod lexer;
mod parser;

use lexer::Token;
use logos::Logos;
use parser::Parser;

fn main() {
    let source = r#"
        print("Hello, World!\n");
        let x = 10 + 2;
        print(x);
        let y = 5 * 3;
        print(y);
        let z = 10 / 0;
        print(z);
    "#;

    let lexer = Token::lexer(source);
    let mut parser = Parser::new(lexer);

    if let Some(exprs) = parser.parse() {
        let mut interpreter = interpreter::Interpreter::new();

        for expr in exprs {
            interpreter.interpret(expr);
        }
    } else {
        println!("Parsing Error: Unable to parse the source code.");
    }
}
