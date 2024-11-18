use std::{collections::HashMap, mem};

use crate::{parser::ast::{BinOp, Expr, ExprValue, Type}, typing::error::*};

pub fn annotate(symbols: &HashMap<&str, Type>, expr: &mut Expr) -> Result<(), TypeError> {
    if let Some(_) = &expr.type_ {
        return Ok(());
    }

    match *expr.value {
        ExprValue::Int(_) => {
            expr.type_ = Some(Type::Int);
            Ok(())
        }
        ExprValue::Float(_) => {
            expr.type_ = Some(Type::Float);
            Ok(())
        },
        ExprValue::Boolean(_) => {
            expr.type_ = Some(Type::Boolean);
            Ok(())
        } 
        // Expr::StringLiteral(_) => Ok(Expr::TypedExpr(Type::String, expr.clone())),
        // Expr::Identifier(id) => {
        //     match ctx.get(id.as_str()) {
        //         Some(t) => Ok(Expr::TypedExpr(t.clone(), Box::new(expr.clone()))),
        //         None => Err(TypeError::UnknownIdentifier(id.clone(), expr.clone())),
        //     }
        // },
        ExprValue::BinOp(ref mut l, ref op, ref mut r) => {
          
            annotate(symbols, l)?;
            annotate(symbols, r)?;

            let Some(ltype) = l.type_.clone() else {
                return Err(TypeError::TypeAnnotationError(l.clone()));
            };
            let Some(rtype) = r.type_.clone() else {
                return Err(TypeError::TypeAnnotationError(r.clone()));
            };

            let mut result_type = ltype.clone();

            // Both sides must have the same type (at least for now)
            if ltype != rtype {
                return Err(TypeError::TypeMismatch(expr.clone(), ltype, rtype));
            }

            // If binop is comparison, resulting type will always be boolean
            let op = op.clone();
            if op == BinOp::Eq || op == BinOp::Ne || op == BinOp::Lt || op == BinOp::Gt || op == BinOp::Ge || op == BinOp::Le {
                result_type = Type::Boolean;
            }

            expr.type_ = Some(result_type);
            Ok(())
        },
        ExprValue::UnOp(_, ref mut r) => {
            annotate(symbols, r)?;
            if r.type_.is_none() {
                return Err(TypeError::TypeAnnotationError(expr.clone()));
            }
            let rtype = r.type_.as_ref().unwrap().clone();
            expr.type_ = Some(rtype);
            Ok(())
        },
    }
} 
