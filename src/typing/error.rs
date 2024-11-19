use crate::parser::ast::{Expr, Type};

#[derive(Debug)]
pub enum TypeError {
    TypeMismatch(Expr, Type, Type),
    UnknownIdentifier(String, Expr),
    TypeAnnotationError(Expr),
}
