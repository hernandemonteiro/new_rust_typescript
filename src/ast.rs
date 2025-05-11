// Representa os tipos de expressões na AST
#[derive(Debug, Clone)]
pub enum Expr {
    // Atribuição: let x = 10
    Assign(String, Box<Expr>),

    // Operações aritméticas: x + 5
    BinOp(Box<Expr>, BinOp, Box<Expr>),

    // Identificador de variável
    Var(String),

    // Números literais
    Number(i64),

    // Função (no nosso caso, print)
    Call(String, Vec<Expr>),

    // Representa uma chamada de função para imprimir
    Print(Box<Expr>),
}

// Representa os tipos de operações binárias
#[derive(Debug, Clone)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

impl Expr {
    pub fn eval(&self) -> Option<i64> {
        match self {
            Expr::Number(n) => Some(*n),
            Expr::Var(name) => {
                // Lógica para variáveis pode ser adicionada aqui, se necessário
                None
            }
            Expr::BinOp(left, op, right) => {
                let left_val = left.eval()?;
                let right_val = right.eval()?;
                match op {
                    BinOp::Add => Some(left_val + right_val),
                    BinOp::Sub => Some(left_val - right_val),
                    BinOp::Mul => Some(left_val * right_val),
                    BinOp::Div => Some(left_val / right_val),
                }
            }
            Expr::Assign(name, value) => {
                // Se você quiser adicionar suporte a variáveis, pode manipular o valor da variável aqui
                value.eval()
            }
            Expr::Print(expr) => {
                if let Some(val) = expr.eval() {
                    println!("{}", val); // Imprime o valor da expressão
                    return Some(val); // Retorna o valor impresso
                }
                None
            }
            Expr::Call(name, args) => {
                // Lógica para chamadas de função, se necessário
                None
            }
        }
    }
}
