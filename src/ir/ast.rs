use std::collections::HashMap;


#[derive(Clone, Debug, PartialEq)]
pub enum UnOp {
    Not,
    // Neg,
    // Pos 
}

#[derive(Clone, Debug, PartialEq)]
pub enum BinOp {
    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    Mod,

    // Logical
    And,
    Or,

    // Comparison
    Eq,
    Ne,
    Lt,
    Gt,
    Ge,
    Le,

    // Member access
    // Dot,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    Int,
    Float,
    // String,
    Boolean,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    // StringLiteral(String),
    // Identifier(String),
    Int(i64),
    Float(f64),
    BinOp(Box<Expr>, BinOp, Box<Expr>),
    UnOp(UnOp, Box<Expr>),
    Boolean(bool),
    TypedExpr(Type, Box<Expr>),
    // Apply(String, Vec<Arg>),
}

pub fn binop(l: &Expr, op: &BinOp, r: &Expr) -> Expr {
    Expr::BinOp(Box::new(l.clone()), op.clone(), Box::new(r.clone()))
}

pub fn unop(op: &UnOp, r: &Expr) -> Expr {
    Expr::UnOp(op.clone(), Box::new(r.clone()))
}
