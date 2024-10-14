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


pub enum TypeError {
    TypeMismatch(Expr, Type, Type),
    UnknownIdentifier(String, Expr),
    TypeAnnotationError(Expr),
}

pub fn unwrap_typed_expr(expr: &Expr) -> Option<(Expr, Type)> {
    match expr {
        Expr::TypedExpr(t, e) => Some((*e.clone(), t.clone())),
        _ => None,
    }
}

pub fn annotate_types(ctx: &HashMap<&str, Type>, expr: &Expr) -> Result<Expr, TypeError> {
    match expr {
        Expr::Int(_) => Ok(Expr::TypedExpr(Type::Int, Box::new(expr.clone()))),
        Expr::Float(_) => Ok(Expr::TypedExpr(Type::Float, Box::new(expr.clone()))),
        // Expr::StringLiteral(_) => Ok(Expr::TypedExpr(Type::String, expr.clone())),
        Expr::Boolean(_) => Ok(Expr::TypedExpr(Type::Boolean, Box::new(expr.clone()))),
        // Expr::Identifier(id) => {
        //     match ctx.get(id.as_str()) {
        //         Some(t) => Ok(Expr::TypedExpr(t.clone(), Box::new(expr.clone()))),
        //         None => Err(TypeError::UnknownIdentifier(id.clone(), expr.clone())),
        //     }
        // },
        Expr::BinOp(l, op, r) => {
            let ltyped = annotate_types(ctx, l)?;
            let rtyped = annotate_types(ctx, r)?;

            let (_, ltype) = unwrap_typed_expr(&ltyped)
                .ok_or_else(|| TypeError::TypeAnnotationError(expr.clone()))?;
            let (_, rtype) = unwrap_typed_expr(&rtyped)
                .ok_or_else(|| TypeError::TypeAnnotationError(expr.clone()))?;

            let mut result_type = ltype.clone();
            if ltype != rtype {
                // Implicit type conversion for floats and ints
                if (ltype == Type::Float && rtype == Type::Int) || (ltype == Type::Int && rtype == Type::Float) {
                    result_type = Type::Float;
                } else {
                    return Err(TypeError::TypeMismatch(expr.clone(), ltype, rtype));
                }
            }
            Ok(Expr::TypedExpr(result_type, Box::new(binop(&ltyped, &op, &rtyped))))
        },
        Expr::UnOp(op, r) => {
            let rtyped = annotate_types(ctx, r)?;
            let (_, rtype) = unwrap_typed_expr(&rtyped)
                .ok_or_else(|| TypeError::TypeAnnotationError(expr.clone()))?;
            Ok(Expr::TypedExpr(rtype, Box::new(unop(&op, &rtyped))))
        },
        Expr::TypedExpr(_, _) => Ok(expr.clone()),
    }
} 

