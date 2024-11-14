use crate::parser::ast::{Expr, Type};

pub enum TypeError {
    TypeMismatch(Expr, Type, Type),
    UnknownIdentifier(String, Expr),
    TypeAnnotationError(Expr),
}
