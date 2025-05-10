use std::collections::HashMap;

use crate::parser::ast::*;
use crate::typing;
use crate::typing::error::TypeError;

use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;

#[derive(Debug)]
pub struct CodeGen<'a, 'b> {
    pub module: &'a Module<'b>,
    pub builder: &'a Builder<'b>,
    pub context: &'a Context,
}

#[derive(Debug)]
pub enum CompileError {
    TypeError(TypeError),
    UntypedExpr(Expr),
    BinopNotSupportedForType(BinOp, Type),
    ExprMustBeInt(Expr),
    ExprMustBeFloat(Expr),
    BuilderError(inkwell::builder::BuilderError)
}

pub enum CompiledExpr<'a> {
    IntValue(inkwell::values::IntValue<'a>),
    FloatValue(inkwell::values::FloatValue<'a>),
}

fn build_intval<'a>(expr: Result<inkwell::values::IntValue<'a>, inkwell::builder::BuilderError>) -> Result<CompiledExpr<'a>, CompileError> {
    let cexpr = expr.or_else(|e| Err(CompileError::BuilderError(e)))?;
    Ok(CompiledExpr::IntValue(cexpr))
}

fn build_floatval<'a>(expr: Result<inkwell::values::FloatValue<'a>, inkwell::builder::BuilderError>) -> Result<CompiledExpr<'a>, CompileError> {
    let cexpr = expr.or_else(|e| Err(CompileError::BuilderError(e)))?;
    Ok(CompiledExpr::FloatValue(cexpr))
}

pub fn expr<'a, 'b>(symbols: &HashMap<&str, Type>, codegen: &CodeGen<'a, 'b>, expression: &Expr) -> Result<CompiledExpr<'a>, CompileError> {

    let Some(t) = &expression.type_ else {
        return Err(CompileError::UntypedExpr(expression.clone()));
    };

    match *expression.value {
        ExprValue::Int(i) => Ok(CompiledExpr::IntValue(codegen.context.i64_type().const_int(i as u64, false))),
        ExprValue::Float(f) => Ok(CompiledExpr::FloatValue(codegen.context.f64_type().const_float(f))),
        ExprValue::Boolean(b) => Ok(CompiledExpr::IntValue(codegen.context.bool_type().const_int(b as u64, false))),
        ExprValue::UnOp(ref op, ref ex) => {
            let e = expr(&symbols, &codegen, &ex)?;
            match t {
                Type::Boolean => {
                    let CompiledExpr::IntValue(v) = e else { 
                        Err(CompileError::ExprMustBeInt(ex.clone()))?
                    };

                    match op {
                        UnOp::Not => build_intval(codegen.builder.build_not(v, "neg_bool")),
                    }
                },
                _ => unimplemented!(),
            }
            
        },
        ExprValue::BinOp(ref lhs, ref op, ref rhs) => {
            let l = expr(&symbols, &codegen, &lhs)?;
            let r = expr(&symbols, &codegen, &rhs)?;
 
            match t {
                Type::Int => {

                    let CompiledExpr::IntValue(lv) = l else { 
                        Err(CompileError::ExprMustBeInt(lhs.clone()))?
                    };

                    let CompiledExpr::IntValue(rv) = r else { 
                        Err(CompileError::ExprMustBeInt(rhs.clone()))?
                    };

                    match op {
                        BinOp::Add => build_intval(codegen.builder.build_int_add(lv, rv, "add_int")),
                        BinOp::Sub => build_intval(codegen.builder.build_int_sub(lv, rv, "sub_int")),
                        BinOp::Mul => build_intval(codegen.builder.build_int_mul(lv, rv, "mul_int")),
                        BinOp::Div => build_intval(codegen.builder.build_int_signed_div(lv, rv, "signed_div_int")),
                        BinOp::Mod => build_intval(codegen.builder.build_int_signed_rem(lv, rv, "mod_int")),
                        BinOp::Eq => build_intval(codegen.builder.build_int_compare(inkwell::IntPredicate::EQ, lv, rv, "eq_int")),
                        BinOp::Ne => build_intval(codegen.builder.build_int_compare(inkwell::IntPredicate::NE, lv, rv, "ne_int")),
                        BinOp::Lt => build_intval(codegen.builder.build_int_compare(inkwell::IntPredicate::SLT, lv, rv, "lt_int")),
                        BinOp::Gt => build_intval(codegen.builder.build_int_compare(inkwell::IntPredicate::SGT, lv, rv, "gt_int")),
                        BinOp::Le => build_intval(codegen.builder.build_int_compare(inkwell::IntPredicate::SLE, lv, rv, "le_int")),
                        BinOp::Ge => build_intval(codegen.builder.build_int_compare(inkwell::IntPredicate::SGE, lv, rv, "ge_int")),
                        _ => Err(CompileError::BinopNotSupportedForType(op.clone(), Type::Int)), 
                    }
                },
                Type::Float => {
                    let CompiledExpr::FloatValue(lv) = l else { 
                        Err(CompileError::ExprMustBeFloat(lhs.clone()))?
                    };

                    let CompiledExpr::FloatValue(rv) = l else { 
                        Err(CompileError::ExprMustBeFloat(rhs.clone()))?
                    };

                    match op {
                        BinOp::Add => build_floatval(codegen.builder.build_float_add(lv, rv, "add_float")),
                        BinOp::Sub => build_floatval(codegen.builder.build_float_sub(lv, rv, "sub_float")),
                        BinOp::Mul => build_floatval(codegen.builder.build_float_mul(lv, rv, "mul_float")),
                        BinOp::Div => build_floatval(codegen.builder.build_float_div(lv, rv, "div_float")),
                        BinOp::Mod => build_floatval(codegen.builder.build_float_rem(lv, rv, "mod_float")),
                        BinOp::Eq => build_intval(codegen.builder.build_float_compare(inkwell::FloatPredicate::OEQ, lv, rv, "eq_float")),
                        BinOp::Ne => build_intval(codegen.builder.build_float_compare(inkwell::FloatPredicate::ONE, lv, rv, "ne_float")),
                        BinOp::Lt => build_intval(codegen.builder.build_float_compare(inkwell::FloatPredicate::OLT, lv, rv, "lt_float")),
                        BinOp::Gt => build_intval(codegen.builder.build_float_compare(inkwell::FloatPredicate::OGT, lv, rv, "gt_float")),
                        BinOp::Le => build_intval(codegen.builder.build_float_compare(inkwell::FloatPredicate::OLE, lv, rv, "le_float")),
                        BinOp::Ge => build_intval(codegen.builder.build_float_compare(inkwell::FloatPredicate::OGE, lv, rv, "ge_float")),
                        _ => Err(CompileError::BinopNotSupportedForType(op.clone(), Type::Float)),
                    }
                },
                Type::Boolean => {
                    let CompiledExpr::IntValue(lv) = l else { 
                        Err(CompileError::ExprMustBeInt(lhs.clone()))?
                    };

                    let CompiledExpr::IntValue(rv) = l else { 
                        Err(CompileError::ExprMustBeInt(rhs.clone()))?
                    };

                    match op {
                        BinOp::Eq => build_intval(codegen.builder.build_int_compare(inkwell::IntPredicate::EQ, lv, rv, "eq_bool")),
                        BinOp::Ne => build_intval(codegen.builder.build_int_compare(inkwell::IntPredicate::NE, lv, rv, "ne_bool")),
                        BinOp::Lt => build_intval(codegen.builder.build_int_compare(inkwell::IntPredicate::SLT, lv, rv, "lt_bool")),
                        BinOp::Gt => build_intval(codegen.builder.build_int_compare(inkwell::IntPredicate::SGT, lv, rv, "gt_bool")),
                        BinOp::Le => build_intval(codegen.builder.build_int_compare(inkwell::IntPredicate::SLE, lv, rv, "le_bool")),
                        BinOp::Ge => build_intval(codegen.builder.build_int_compare(inkwell::IntPredicate::SGE, lv, rv, "ge_bool")),
                        _ => Err(CompileError::BinopNotSupportedForType(op.clone(), Type::Boolean)),
                    }
                },
                _ => unimplemented!(),
            }
        }
    }
}

pub fn block(symbols: &HashMap<&str, Type>, codegen: &CodeGen, block: &Block) -> Result<(), CompileError> {
    for stmt in &block.stmts {
        stmt(&symbols, &codegen)?;
    }

    Ok(())
}

// TODO this is using .clone(). Refactor to reference instead.
pub fn get_basic_value<'a>(compiled_expr: &CompiledExpr<'a>) -> Box<dyn inkwell::values::BasicValue<'a> + 'a> {
    match compiled_expr {
        CompiledExpr::IntValue(v) => Box::new(v.clone()),
        CompiledExpr::FloatValue(v) => Box::new(v.clone()),
    }
}
