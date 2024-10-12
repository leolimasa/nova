#[derive(Clone, Debug, PartialEq)]
pub enum UnOp {
    Not,
    Neg,
    Pos 
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
    Dot,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Expr {
    pub value: Box<ExprValue>,
}

impl Expr {
   pub fn new(value: ExprValue) -> Expr {
       Expr {
           value: Box::new(value)
       }
   }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExprValue {
    StringLiteral(String),
    Identifier(String),
    Int(i64),
    Float(f64),
    BinOp(Box<Expr>, BinOp, Box<Expr>),
    UnOp(UnOp, Box<Expr>),
    Boolean(bool),
    // Apply(String, Vec<Arg>),
}

