
pub enum TypeError {
    TypeMismatch(Expr, Type, Type),
    UnknownIdentifier(String, Expr),
    TypeAnnotationError(Expr),
}
