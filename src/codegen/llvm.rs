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
    BinopNotSupportedForType(BinOp, Type),
    ExprMustBeInt(Expr),
    BuilderError(inkwell::builder::BuilderError)
}

pub enum CompiledExpr<'a> {
    IntValue(inkwell::values::IntValue<'a>),
    FloatValue(inkwell::values::FloatValue<'a>),
}

fn build_intval<'a, T>(expr: Result<inkwell::values::IntValue<'a>, inkwell::builder::BuilderError>) -> Result<CompiledExpr<'a>, CompileError> {
    let cexpr = expr.or_else(|e| Err(CompileError::BuilderError(e)))?;
    Ok(CompiledExpr::IntValue(cexpr))
}

pub fn expr<'a>(symbols: &HashMap<&str, Type>, codegen: &CodeGen, expression: &Expr) -> Result<CompiledExpr<'a>, CompileError> {

    let Some(t) = expression.type_ else {
        return Err(CompileError::UntypedExpr(expression.clone()));
    };

    match *expression.value {
        ExprValue::Int(i) => Ok(CompiledExpr::IntValue(codegen.context.i64_type().const_int(i as u64, false))),
        ExprValue::Float(f) => Ok(CompiledExpr::FloatValue(codegen.context.f64_type().const_float(f))),
        ExprValue::Boolean(b) => Ok(CompiledExpr::IntValue(codegen.context.bool_type().const_int(b as u64, false))),
        ExprValue::BinOp(lhs, op, rhs) => {
            let l = expr(&symbols, &codegen, &lhs)?;
            let r = expr(&symbols, &codegen, &rhs)?;
 
            match t {
                Type::Int => {

                    let CompiledExpr::IntValue(lv) = l else { 
                        Err(CompileError::ExprMustBeInt(lhs.clone()))?
                    };

                    let CompiledExpr::IntValue(rv) = l else { 
                        Err(CompileError::ExprMustBeInt(rhs.clone()))?
                    };

                    match op {
                        BinOp::Add => build_intval(codegen.builder.build_int_add(lv, rv, "add_int")),
                        BinOp::Sub => build_intval(codegen.builder.build_int_sub(lv, rv, "sub_int")),
                        BinOp::Mul => codegen.builder.build_mul(),
                        BinOp::Div => codegen.builder.build_div(),
                        BinOp::Mod => codegen.builder.build_mod(),
                        BinOp::Eq => codegen.builder.build_eq(),
                        BinOp::Ne => codegen.builder.build_ne(),
                        BinOp::Lt => codegen.builder.build_lt(),
                        BinOp::Gt => codegen.builder.build_gt(),
                        BinOp::Le => codegen.builder.build_le(),
                        BinOp::Ge => codegen.builder.build_ge(),
                        _ => Err(CompileError::BinopNotSupportedForType(op, Type::Int)), 
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
                        _ => Err(CompileError::BinopNotSupportedForType(op, Type::Float)),
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
                        _ => Err(CompileError::BinopNotSupportedForType(op, Type::Boolean)),
                    }
                },
                _ => unimplemented!(),
            }
        }

    }
}
