use std::collections::HashMap;

use crate::parser::ast::*;
use crate::typing;
use crate::typing::error::TypeError;

use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;

struct CodeGen<'a> {
    module: &'a Module<'a>,
    builder: &'a Builder<'a>,
    context: &'a Context,
}

pub enum CompileError {
    TypeError(TypeError),
    UntypedExpr(Expr),
}

pub fn expr(symbols: &HashMap<&str, Type>, codegen: &CodeGen, expression: &Expr) -> Result<(), CompileError> {
    let typed_expr = typing::expr::annotate(&symbols, &expression).or_else(|e| Err(CompileError::TypeError(e)))?; 
   
    let type = typed_expr.type_.ok_or_else(|| CompileError::UntypedExpr(expression.clone()))?;

    match *typed_expr.value {
        ExprValue::Int(i) => codegen.context.i64_type().const_int(i as u64, false),
        ExprValue::Float(f) => codegen.builder.build_float(f),
        ExprValue::Boolean(b) => codegen.builder.build_boolean(b),
        ExprValue::BinOp(lhs, op, rhs) => {
            expr(&symbols, &codegen, &lhs)?;
            expr(&symbols, &codegen, &rhs)?;
 
            match type {
                Type::Int => {
                    match op {
                        BinOp::Add => codegen.context.build_add(),
                        BinOp::Sub => codegen.builder.build_sub(),
                        BinOp::Mul => codegen.builder.build_mul(),
                        BinOp::Div => codegen.builder.build_div(),
                        BinOp::Mod => codegen.builder.build_mod(),
                        BinOp::Eq => codegen.builder.build_eq(),
                        BinOp::Ne => codegen.builder.build_ne(),
                        BinOp::Lt => codegen.builder.build_lt(),
                        BinOp::Gt => codegen.builder.build_gt(),
                        BinOp::Le => codegen.builder.build_le(),
                        BinOp::Ge => codegen.builder.build_ge(),
                    }
                },
                Type::Float => {
                    match op {
                        BinOp::Add => codegen.builder.build_fadd(),
                        BinOp::Sub => codegen.builder.build_fsub(),
                        BinOp::Mul => codegen.builder.build_fmul(),
                        BinOp::Div => codegen.builder.build_fdiv(),
                        BinOp::Mod => codegen.builder.build_fmod(),
                        BinOp::Eq => codegen.builder.build_feq(),
                        BinOp::Ne => codegen.builder.build_fne(),
                        BinOp::Lt => codegen.builder.build_flt(),
                        BinOp::Gt => codegen.builder.build_fgt(),
                        BinOp::Le => codegen.builder.build_fle(),
                        BinOp::Ge => codegen.builder.build_fge(),
                    }
                },
                Type::Boolean => {
                    match op {
                        BinOp::Eq => codegen.builder.build_eq(),
                        BinOp::Ne => codegen.builder.build_ne(),
                        BinOp::Lt => codegen.builder.build_lt(),
                        BinOp::Gt => codegen.builder.build_gt(),
                        BinOp::Le => codegen.builder.build_le(),
                        BinOp::Ge => codegen.builder.build_ge(),
                    }
                },
                _ => unimplemented!(),
            }
        }

    }
    
}
