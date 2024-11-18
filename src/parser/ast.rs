
#[derive(Clone, Debug, PartialEq)]
pub struct Loc {
    pub start: usize,
    pub end: usize,
}

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
pub struct Expr {
    pub type_: Option<Type>,
    pub loc: Loc,
    pub value: Box<ExprValue>, 
}

pub fn empty_expr() -> Expr {
    Expr { 
        type_: None, 
        loc: Loc {start:0, end: 0},
        value: Box::new(ExprValue::Int(0))
    }
}

pub fn expr(type_: Option<Type>, value: ExprValue, start: usize, end: usize) -> Expr {
    Expr {
        type_,
        loc: Loc { start, end },
        value: Box::new(value),
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExprValue {
    // StringLiteral(String),
    // Identifier(String),
    Int(i64),
    Float(f64),
    BinOp(Expr, BinOp, Expr),
    UnOp(UnOp, Expr),
    Boolean(bool),
    // Apply(String, Vec<Arg>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Statement {
    pub type_: Option<Type>,
    pub loc: Loc,
    pub value: StatementValue,
}

#[derive(Clone, Debug, PartialEq)]
pub enum StatementValue {
    Expr(Expr),
    Return(Expr),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Block {
    pub statements: Vec<Statement>, 
    pub type_: Option<Type>,
    pub loc: Loc,
}

pub fn block(statements: Vec<Statement>, start: usize, end: usize) -> Block {
    Block {
        statements,
        type_: None,
        loc: Loc { start, end },
    }
}


#[derive(Clone, Debug, PartialEq)]
pub struct Ident {
    pub name: String,
    pub type_: Option<Type>,
    pub loc: Loc,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Function {
    pub name: Option<String>,
    pub args: Vec<Ident>,
    pub body: Block,
    pub return_type: Type,
    pub loc: Loc,
}

pub fn set_fun_name(name: String, fun: Function) -> Function {
    Function {
        name: Some(name),
        ..fun
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Module {
    pub statements: Vec<ModuleStatement>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ModuleStatement {
    pub value: ModuleStatementValue,
    pub loc: Loc,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ModuleStatementValue {
    Function(Function),
    // Variable(Ident),
    // Import(String),
}

pub fn binop(l: &Expr, op: &BinOp, r: &Expr, start: usize, end: usize) -> Expr {
    Expr {
        type_: None,
        loc: Loc { start, end },
        value: Box::new(ExprValue::BinOp(l.clone(), op.clone(), r.clone())),
    }
}

pub fn unop(op: &UnOp, r: &Expr, start: usize, end: usize) -> Expr {
    Expr {
        type_: None,
        loc: Loc { start, end },
        value: Box::new(ExprValue::UnOp(op.clone(), r.clone())),
    }
}
