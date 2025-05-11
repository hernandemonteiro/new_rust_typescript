mod ast;
mod interpreter;
mod lexer;
mod parser;

use lexer::Token;
use logos::Logos;
use parser::Parser;

fn main() {
    let source = r#"
        let x = 10 + 2;
        print(x);
    "#;

    let lexer = Token::lexer(source);
    let mut parser = Parser::new(lexer);

    if let Some(ast) = parser.parse() {
        let mut interpreter = interpreter::Interpreter::new();

        interpreter.interpret(ast);
    } else {
        println!("Erro ao fazer parsing");
    }
}
