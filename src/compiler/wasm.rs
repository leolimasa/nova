use wasm_encoder::{
    CodeSection, ExportKind, ExportSection, Function, FunctionSection, Instruction,
    Module, TypeSection, ValType,
};

use crate::ir::ast::{
    unwrap_typed_expr, Expr, BinOp, UnOp, Type
};

pub enum WasmCompileError {
    UntypedExpr(Expr),
    BinopNotSupportedForType(BinOp, Expr),
    UnopNotSupportedForType(UnOp, Expr),
}

pub fn compile_expr<'a>(typed_expr: Box<Expr>) -> Result<Vec<Instruction<'a>>, WasmCompileError> {
    let (expr, expr_type) = unwrap_typed_expr(&typed_expr)
        .ok_or_else(|| WasmCompileError::UntypedExpr(*typed_expr.clone()))?;

    match expr {
        Expr::Int(n) => {
            Ok(vec![Instruction::I64Const(n)])
        }
        Expr::Float(n) => {
            Ok(vec![Instruction::F64Const(n)])
        }
        Expr::Boolean(b) => {
            Ok(vec![Instruction::I32Const(if b { 1 } else { 0 })])
        }
        Expr::BinOp(l, op, r) => {
            // ---- Left hand -----
            let (_, ltype) = unwrap_typed_expr(&l)
                .ok_or_else(|| WasmCompileError::UntypedExpr(*typed_expr.clone()))?;
            let mut instructions = compile_expr(l)?;
            // Cast to left to float if needed
            if expr_type == Type::Float && ltype == Type::Int {
                instructions.push(Instruction::F64ConvertI64S)
            }

            // ---- Right hand -----
            let (_, rtype) = unwrap_typed_expr(&r)
                .ok_or_else(|| WasmCompileError::UntypedExpr(*typed_expr.clone()))?;
            instructions.extend(compile_expr(r)?);
            // Cast to right to float if needed
            if expr_type == Type::Float && rtype == Type::Int {
                instructions.push(Instruction::F64ConvertI64S)
            }

            match expr_type {
                Type::Int => {
                    instructions.extend(match op {
                        BinOp::Add => vec![Instruction::I64Add],
                        BinOp::Sub => vec![Instruction::I64Sub],
                        BinOp::Mul => vec![Instruction::I64Mul],
                        BinOp::Div => vec![Instruction::I64DivU],
                        BinOp::Eq => vec![Instruction::I64Eq],
                        BinOp::Ne => vec![Instruction::I64Ne],
                        BinOp::Lt => vec![Instruction::I64LtS],
                        BinOp::Le => vec![Instruction::I64LeS],
                        BinOp::Gt => vec![Instruction::I64GtS],
                        BinOp::Ge => vec![Instruction::I64GeS],
                        BinOp::Mod => vec![Instruction::I64RemU],
                        _ => Err(WasmCompileError::BinopNotSupportedForType(op, *typed_expr.clone()))?,
                    });
                }
                Type::Float => {
                    instructions.extend(match op {
                        BinOp::Add => vec![Instruction::F64Add],
                        BinOp::Sub => vec![Instruction::F64Sub],
                        BinOp::Mul => vec![Instruction::F64Mul],
                        BinOp::Div => vec![Instruction::F64Div],
                        BinOp::Eq => vec![Instruction::F64Eq],
                        BinOp::Ne => vec![Instruction::F64Ne],
                        BinOp::Lt => vec![Instruction::F64Lt],
                        BinOp::Le => vec![Instruction::F64Le],
                        BinOp::Gt => vec![Instruction::F64Gt],
                        BinOp::Ge => vec![Instruction::F64Ge],
                        _ => Err(WasmCompileError::BinopNotSupportedForType(op, *typed_expr.clone()))?,
                    });
                }
                Type::Boolean => {
                    instructions.extend(match op {
                        BinOp::Eq => vec![Instruction::I32Eq],
                        BinOp::Ne => vec![Instruction::I32Ne],
                        BinOp::Lt => vec![Instruction::I32LtS],
                        BinOp::Le => vec![Instruction::I32LeS],
                        BinOp::Gt => vec![Instruction::I32GtS],
                        BinOp::Ge => vec![Instruction::I32GeS],
                        _ => Err(WasmCompileError::BinopNotSupportedForType(op, *typed_expr.clone()))?,
                    });
                }
            }
            Ok(instructions)
        }
        Expr::UnOp(op, r) => {
            match expr_type {
                Type::Boolean => {
                    let mut instructions = compile_expr(r)?;
                    instructions.extend(match op {
                        UnOp::Not => vec![Instruction::I32Eqz],
                    });
                    Ok(instructions)
                },
                _ => Err(WasmCompileError::UnopNotSupportedForType(op, *typed_expr.clone()))?,
            }
        },
        _ => Err(WasmCompileError::UntypedExpr(*typed_expr.clone())),
    }
}

pub fn module_from_expr(expr: &Expr) -> Result<Module, WasmCompileError> {
    let mut module = Module::new();

    // ---- Type section ----
    let (_, expr_type) = unwrap_typed_expr(&expr)
        .ok_or_else(|| WasmCompileError::UntypedExpr(expr.clone()))?;
    
    let return_type = match expr_type {
        Type::Int => ValType::I64,
        Type::Float => ValType::F64,
        Type::Boolean => ValType::I32,
    };

    let mut types = TypeSection::new();
    let params = vec![];
    let results = vec![return_type];
    let main_func_type_idx = 0;
    types.ty().function(params, results);
    module.section(&types);

    // ---- Function section ----
    let mut functions = FunctionSection::new();
    let main_func_idx = 0;
    functions.function(main_func_type_idx);
    module.section(&functions);

    // ---- Export section ----
    let mut exports = ExportSection::new();
    exports.export("main", ExportKind::Func, main_func_idx);
    module.section(&exports);

    // ---- Code section ----
    let mut codes = CodeSection::new();
    let locals = vec![];
    let mut f = Function::new(locals);
    let instructions = compile_expr(Box::new(expr.clone()))?;
    for instr in instructions {
        f.instruction(&instr);
    }
    codes.function(&f);
    module.section(&codes);

    Ok(module)
}