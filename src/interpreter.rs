use crate::ast::{BinOp, Expr};
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

    // Método interpretador
    pub fn interpret(&mut self, ast: Expr) {
        match ast {
            Expr::Assign(var, value) => {
                let evaluated_value = self.eval(*value);
                self.variables.insert(var.clone(), evaluated_value);
                println!("{}", evaluated_value);
            }
            Expr::Var(var) => {
                if let Some(value) = self.variables.get(&var) {
                    println!("Valor de '{}': {}", var, value);
                } else {
                    println!("Variável '{}' não encontrada", var);
                }
            }
            Expr::Number(value) => {
                println!("Número: {}", value);
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
                            println!("Erro: Divisão por zero");
                            0
                        }
                    }
                };

                println!("{}", result);
            }
            Expr::Print(expr) => {
                let value = self.eval(*expr);
                println!("Impressão: {}", value);
            }
            Expr::Call(func_name, args) => {
                if func_name == "print" {
                    if let Some(arg) = args.get(0) {
                        let value = self.eval(arg.clone());
                        println!("Função '{}', argumento: {}", func_name, value);
                    }
                } else {
                    println!("Função desconhecida: {}", func_name);
                }
            }
        }
    }

    // Método de avaliação
    pub fn eval(&mut self, expr: Expr) -> i64 {
        match expr {
            Expr::Number(value) => value,
            Expr::Var(var) => {
                let value = *self.variables.get(&var).unwrap_or(&0);
                value
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
                            println!("Erro: Divisão por zero");
                            0
                        }
                    }
                };
                result
            }
            _ => {
                println!("Resultado padrão (não tratado): 0");
                0
            }
        }
    }
}
