
pub fn annotate(ctx: &HashMap<&str, Type>, expr: &Expr) -> Result<Expr, TypeError> {
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

            // If binop is comparison, resulting type will always be boolean
            if *op == BinOp::Eq || *op == BinOp::Ne || *op == BinOp::Lt || *op == BinOp::Gt || *op == BinOp::Ge || *op == BinOp::Le {
                result_type = Type::Boolean;
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
