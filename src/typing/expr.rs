use std::collections::HashMap;

use crate::{parser::ast::{BinOp, Expr, ExprValue, Type}, typing::error::*};

pub fn annotate(symbols: &HashMap<&str, Type>, expr: &Expr) -> Result<Expr, TypeError> {

    if let Some(t) = expr.type_ {
        return Ok(expr.clone());
    }

    match *expr.value {
        ExprValue::Int(_) => Ok(Expr { type_: Some(Type::Int), ..expr.clone() }),
        ExprValue::Float(_) => Ok(Expr { type_: Some(Type::Float), ..expr.clone() }),
        ExprValue::Boolean(_) => Ok(Expr { type_: Some(Type::Boolean), ..expr.clone() }),
        // Expr::StringLiteral(_) => Ok(Expr::TypedExpr(Type::String, expr.clone())),
        // Expr::Identifier(id) => {
        //     match ctx.get(id.as_str()) {
        //         Some(t) => Ok(Expr::TypedExpr(t.clone(), Box::new(expr.clone()))),
        //         None => Err(TypeError::UnknownIdentifier(id.clone(), expr.clone())),
        //     }
        // },
        ExprValue::BinOp(l, op, r) => {
            let ltyped = annotate(symbols, &l)?;
            let rtyped = annotate(symbols, &r)?;
            
            let ltype = ltyped.type_.ok_or_else(|| TypeError::TypeAnnotationError(expr.clone()))?;
            let rtype = rtyped.type_.ok_or_else(|| TypeError::TypeAnnotationError(expr.clone()))?;
            let mut result_type = ltype.clone();

            if ltype != rtype {
                return Err(TypeError::TypeMismatch(expr.clone(), ltype, rtype));
            }

            // If binop is comparison, resulting type will always be boolean
            if op == BinOp::Eq || op == BinOp::Ne || op == BinOp::Lt || op == BinOp::Gt || op == BinOp::Ge || op == BinOp::Le {
                result_type = Type::Boolean;
            }

            Ok(Expr { type_: Some(result_type), ..expr.clone() })
        },
        ExprValue::UnOp(op, r) => {
            let rtyped = annotate(symbols, &r)?;
            let rtype = rtyped.type_.ok_or_else(|| TypeError::TypeAnnotationError(expr.clone()))?;
            Ok(Expr { type_: Some(rtype), ..expr.clone() })
        },
    }
} 
