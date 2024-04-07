
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
    Op(Box<Expr>, Opcode, Box<Expr>),
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
pub struct FunDef {
    pub name: Option<String>,
    pub args: Vec<Arg>,
    pub body: Block,
}

#[derive(Clone,Debug)]
pub enum Opcode {
    Add,
    Sub,
    Mul,
    Div,
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
            value: value,
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
