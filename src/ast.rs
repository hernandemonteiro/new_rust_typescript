#[derive(Debug, Clone)]
pub enum Expr {
    Assign(String, Box<Expr>),
    BinOp(Box<Expr>, BinOp, Box<Expr>),
    Var(String),
    Number(i64),
    String(String),
    Call(String, Vec<Expr>),
    Print(Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}
