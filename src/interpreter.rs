use crate::ast::{BinOp, Expr};
use colored::*;
use std::collections::HashMap;

pub struct Interpreter {
    variables: HashMap<String, i64>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            variables: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, ast: Expr) {
        match ast {
            Expr::Assign(var, value) => {
                let evaluated_value = self.eval(*value);
                self.variables.insert(var.clone(), evaluated_value);
            }
            Expr::Var(var) => {
                if let Some(value) = self.variables.get(&var) {
                    println!("Value of '{}': {}", var, value);
                } else {
                    println!("Variable '{}' not found", var);
                }
            }
            Expr::Number(value) => {
                println!("Number: {}", value);
            }
            Expr::String(value) => {
                println!("{}", value);
            }
            Expr::BinOp(left, op, right) => {
                let left_value = self.eval(*left);
                let right_value = self.eval(*right);

                let result = match op {
                    BinOp::Add => left_value + right_value,
                    BinOp::Sub => left_value - right_value,
                    BinOp::Mul => left_value * right_value,
                    BinOp::Div => {
                        if right_value != 0 {
                            left_value / right_value
                        } else {
                            println!("Error: Division by zero");
                            std::process::exit(1);
                        }
                    }
                };

                println!("Operation result: {}", result);
            }
            Expr::Print(expr) => match *expr {
                Expr::String(s) => println!("{}", s),
                _ => {
                    let value = self.eval(*expr);
                    println!("{}", value);
                }
            },
            Expr::Call(func_name, args) => {
                if func_name == "print" {
                    if let Some(arg) = args.get(0) {
                        let value = self.eval(arg.clone());
                        println!("Function '{}', argument: {}", func_name, value);
                    }
                } else {
                    println!("Unknown function: {}", func_name);
                    std::process::exit(1);
                }
            }
        }
    }

    pub fn eval(&mut self, expr: Expr) -> i64 {
        match expr {
            Expr::Number(value) => value,
            Expr::Var(var) => *self.variables.get(&var).unwrap_or(&0),
            Expr::BinOp(left, op, right) => {
                let left_value = self.eval(*left);
                let right_value = self.eval(*right);

                match op {
                    BinOp::Add => left_value + right_value,
                    BinOp::Sub => left_value - right_value,
                    BinOp::Mul => left_value * right_value,
                    BinOp::Div => {
                        if right_value != 0 {
                            left_value / right_value
                        } else {
                            println!("\n{}\n", "Error: Division by zero!".red().bold());
                            std::process::exit(1);
                        }
                    }
                }
            }
            _ => 0,
        }
    }
}
