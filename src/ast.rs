
#[derive(Clone,Debug)]
pub enum Type_ {
    Identifier(String),
    Unknown,
}

#[derive(Clone,Debug)]
pub struct Expr {
    pub value: Box<ExprValue>,
    pub type_: Type_,
}

impl Expr {
   pub fn new(value: ExprValue) -> Expr {
       Expr {
           value: Box::new(value),
           type_: Type_::Unknown,
       }
   }
}

#[derive(Clone,Debug)]
pub enum ExprValue {
    StringLiteral(String),
    Identifier(String),
    Number(i32),
    BinOp(Box<Expr>, BinOp, Box<Expr>),
    UnOp(UnOp, Box<Expr>),
    Apply(String, Vec<Arg>),
}

#[derive(Clone,Debug)]
pub struct Arg {
    pub value: ArgValue,
    pub type_: Type_,
}

impl Arg {
    pub fn new(value: ArgValue) -> Arg {
        Arg {
            value,
            type_: Type_::Unknown,
        }
    }
}

#[derive(Clone,Debug)]
pub enum ArgValue {
    Expr(Box<Expr>),
    FunDef(FunDef),
}

#[derive(Clone,Debug)]
pub struct FunArg {
    pub name: String,
    pub type_: Type_,
}

impl FunArg {
    pub fn new(name: String) -> FunArg {
        FunArg {
            name,
            type_: Type_::Unknown,
        }
    }
}

#[derive(Clone,Debug)]
pub struct FunDef {
    pub name: Option<String>,
    pub args: Vec<FunArg>,
    pub body: Block,
}

#[derive(Clone,Debug)]
pub enum UnOp {
    Not,
    Neg,
    Pos 
}

#[derive(Clone,Debug)]
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

#[derive(Clone,Debug)]
pub enum Statement {
    Expr(Box<Expr>),
    Block(Block),
    Assignment(Assignment),
}

#[derive(Clone,Debug)]
pub struct Assignment {
    pub name: String,
    pub value: AssignmentValue,
    pub type_: Type_,
}

#[derive(Clone,Debug)]
pub enum AssignmentValue {
    Expr(Box<Expr>),
    FunDef(FunDef),
}

impl Assignment {
    pub fn new(name: String, value: AssignmentValue) -> Assignment {
        Assignment {
            name,
            value,
            type_: Type_::Unknown,
        }
    }
}

#[derive(Clone,Debug)]
pub struct Block {
    pub statements: Vec<Statement>,
    pub type_: Type_,
}

impl Block {
    pub fn new(statements: Vec<Statement>) -> Block {
        Block {
            statements,
            type_: Type_::Unknown,
        }
    }

    pub fn single_statement(statement: Statement) -> Block {
        Block {
            statements: vec![statement],
            type_: Type_::Unknown,
        }
    }
}

