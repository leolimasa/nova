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
    Nothing,
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

#[derive(Clone, Debug, PartialEq)]
pub enum Statement {
    Expr(Expr),
    Return(Expr),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Block {
    pub statements: Vec<Statement>, 
    pub type_: Option<Type>,
}

impl Block {
    pub fn new(statements: Vec<Statement>) -> Block {
        Block {
            statements,
            type_: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypedIdent {
    pub name: String,
    pub type_: Type,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Function {
    pub name: String,
    pub args: Vec<TypedIdent>,
    pub body: Block,
    pub return_type: Type,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Module {
    statements: Vec<ModuleStatement>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ModuleStatement {
    FunctionDeclaration(Function),
    Export(String, String),
}

pub fn binop(l: &Expr, op: &BinOp, r: &Expr) -> Expr {
    Expr::BinOp(Box::new(l.clone()), op.clone(), Box::new(r.clone()))
}

pub fn unop(op: &UnOp, r: &Expr) -> Expr {
    Expr::UnOp(op.clone(), Box::new(r.clone()))
}
