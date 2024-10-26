use std::collections::HashMap;

use wasm_encoder::{
    CodeSection, ExportKind, ExportSection, Function, FunctionSection, Instruction, Module,
    TypeSection, ValType,
};

use crate::ir::ast::{BinOp, Block, Expr, Statement, Type, UnOp, Function};

use crate::ir::ast;

use crate::ir::parser::{annotate_types, parse_expr, unwrap_typed_expr, TypeError};

#[derive(Debug)]
pub enum WasmCompileError {
    ExprTypeError(TypeError, Expr),
    UntypedExpr(Expr),
    BinopNotSupportedForType(BinOp, Expr),
    UnopNotSupportedForType(UnOp, Expr),
}

pub fn compile_expr<'a>(typed_expr: Box<Expr>) -> Result<Vec<Instruction<'a>>, WasmCompileError> {
    let (expr, expr_type) = unwrap_typed_expr(&typed_expr)
        .ok_or_else(|| WasmCompileError::UntypedExpr(*typed_expr.clone()))?;

    match expr {
        Expr::Int(n) => Ok(vec![Instruction::I64Const(n)]),
        Expr::Float(n) => Ok(vec![Instruction::F64Const(n)]),
        Expr::Boolean(b) => Ok(vec![Instruction::I32Const(if b { 1 } else { 0 })]),
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

            // Identify the operation type based on the type of the left and right hand expressions.
            // If one is float and the other is int, it will be float.
            // If both are bool, it will be bool. If both are int, it will be int.
            let mut op_type = ltype.clone();
            if ltype == Type::Float || rtype == Type::Float {
                op_type = Type::Float;
            } else if ltype == Type::Boolean && rtype == Type::Boolean {
                op_type = Type::Boolean;
            }

            match op_type {
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
                        _ => Err(WasmCompileError::BinopNotSupportedForType(
                            op,
                            *typed_expr.clone(),
                        ))?,
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
                        _ => Err(WasmCompileError::BinopNotSupportedForType(
                            op,
                            *typed_expr.clone(),
                        ))?,
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
                        BinOp::And => vec![Instruction::I32And],
                        BinOp::Or => vec![Instruction::I32Or],
                        _ => Err(WasmCompileError::BinopNotSupportedForType(
                            op,
                            *typed_expr.clone(),
                        ))?,
                    });
                }
                Type::Nothing => Err(WasmCompileError::BinopNotSupportedForType(
                    op,
                    *typed_expr.clone(),
                ))?,
            }
            Ok(instructions)
        }
        Expr::UnOp(op, r) => match expr_type {
            Type::Boolean => {
                let mut instructions = compile_expr(r)?;
                instructions.extend(match op {
                    UnOp::Not => vec![Instruction::I32Eqz],
                });
                Ok(instructions)
            }
            _ => Err(WasmCompileError::UnopNotSupportedForType(
                op,
                *typed_expr.clone(),
            ))?,
        },
        _ => Err(WasmCompileError::UntypedExpr(*typed_expr.clone())),
    }
}

pub fn module_from_expr(expr: &Expr) -> Result<Module, WasmCompileError> {
    let mut module = Module::new();

    // ---- Type section ----
    let (_, expr_type) =
        unwrap_typed_expr(&expr).ok_or_else(|| WasmCompileError::UntypedExpr(expr.clone()))?;

    let return_type = match expr_type {
        Type::Int => ValType::I64,
        Type::Float => ValType::F64,
        Type::Boolean => ValType::I32,
        Type::Nothing => ValType::I32,
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
    // If the return type is nothing, return 1
    if expr_type == Type::Nothing {
        f.instruction(&Instruction::I32Const(1));
    }
    f.instruction(&Instruction::End);
    codes.function(&f);
    module.section(&codes);

    Ok(module)
}

#[derive(Debug)]
pub enum RunExprError {
    ParseError(String),
    TypeAnnotationError(TypeError),
    WasmCompileError(WasmCompileError),
    WasmtimeError(wasmtime::Error),
}

pub fn run_expr<T>(expr: Box<Expr>) -> Result<T, RunExprError>
where
    T: wasmtime::WasmResults,
{
    let typed_expr = annotate_types(&HashMap::new(), &expr)
        .or_else(|e| Err(RunExprError::TypeAnnotationError(e)))?;

    let module =
        module_from_expr(&typed_expr).or_else(|e| Err(RunExprError::WasmCompileError(e)))?;

    let wasm_bytes = module.finish();

    let engine = wasmtime::Engine::default();
    let wt_module = wasmtime::Module::new(&engine, &wasm_bytes)
        .or_else(|e| Err(RunExprError::WasmtimeError(e)))?;

    // the store contains stuff from the host
    let mut store = wasmtime::Store::new(&engine, ());

    // instantiate the module
    let instance = wasmtime::Instance::new(&mut store, &wt_module, &[])
        .or_else(|e| Err(RunExprError::WasmtimeError(e)))?;

    // get the `main` function from the module and call it
    let main = instance
        .get_typed_func::<(), T>(&mut store, "main")
        .or_else(|e| Err(RunExprError::WasmtimeError(e)))?;

    let result = main
        .call(&mut store, ())
        .or_else(|e| Err(RunExprError::WasmtimeError(e)))?;

    Ok(result)
}

pub fn run_expr_str<T>(expr_str: &str) -> Result<T, RunExprError>
where
    T: wasmtime::WasmResults,
{
    let expr = parse_expr(expr_str).or_else(|e| Err(RunExprError::ParseError(e)))?;
    run_expr(Box::new(expr))
}

fn block_expr_instructions<'a>(
    expr: Expr,
) -> Result<Vec<Instruction<'a>>, WasmCompileError> {
    let mut instructions = vec![];

    let typed_expr = annotate_types(&HashMap::new(), &Box::new(expr.clone()))
        .or_else(|e| Err(WasmCompileError::ExprTypeError(e, expr.clone())))?;

    let (_, expr_type) = unwrap_typed_expr(&typed_expr)
        .ok_or_else(|| WasmCompileError::UntypedExpr(typed_expr.clone()))?;

    instructions.extend(compile_expr(Box::new(typed_expr))?);

    // Drop the result of the expression if it returns anything
    if expr_type != Type::Nothing {
        instructions.push(Instruction::Drop);
    }
    Ok(instructions)
}

pub fn compile_block<'a>(block: Box<Block>) -> Result<Vec<Instruction<'a>>, WasmCompileError> {
    let mut instructions = vec![];

    for statement in block.statements {
        match statement {
            Statement::Expr(expr) => instructions.extend(block_expr_instructions(expr)?),
            Statement::Return(expr) => {
                instructions.extend(block_expr_instructions(expr)?);
                instructions.push(Instruction::Return);
            }
        }
    }
    Ok(instructions)
}

struct CompiledFunction {
    fun: Function,
    params: Vec<ValType>,
    results: Vec<ValType>,
}

// Takes in a Function and returns the bits required to add it
//  to a module
pub fn compile_function(
    function: ast::Function,
) -> Result<CompiledFunction, WasmCompileError> {
    // TODO support arguments
    // TODO support variable assignments

    // Body
    let mut instructions = vec![];
    instructions.extend(compile_block(Box::new(function.body))?);
    instructions.push(Instruction::End);

    let params = function
        .args
        .iter()
        .map(|arg| match arg.type_ {
            Type::Int => ValType::I64,
            Type::Float => ValType::F64,
            Type::Boolean => ValType::I32,
            Type::Nothing => ValType::I32,
        })
        .collect(); 

    let results = match function.return_type {
        Type::Int => vec![ValType::I64],
        Type::Float => vec![ValType::F64],
        Type::Boolean => vec![ValType::I32],
        Type::Nothing => vec![ValType::I32],
    };

    Ok(CompiledFunction { instructions, params, results})
}

pub fn compile_module(module: ast::Module) -> Result<Module, WasmCompileError> {
    let mut module = Module::new();

    let mut types = TypeSection::new();
    let mut functions = FunctionSection::new();
    let mut codes = CodeSection::new();

    module.section(&types);
    module.section(&functions);
    module.section(&codes);

    let mut typeidx = 0;
    let mut funidx = 0;

    for statement in module.statements {
        match statement {
            ast::ModuleStatement::FunctionDeclaration(function) => {
                let compfn = compile_function(function)?;

                // Type
                types.ty().function(compfn.params, compfn.results);
                functions.function(typeidx);

                

            }
        }
    } 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arit_expr() {
        let result = run_expr_str::<f64>("1 + 2.5 * 3 - 4 / 2").unwrap();
        assert_eq!(result, 6.5);

        let result = run_expr_str::<i64>("(1+2)*3").unwrap();
        assert_eq!(result, 9);
    }

    #[test]
    fn test_bool_expr() {
        let result = run_expr_str::<i32>("1 < 2").unwrap();
        assert_eq!(result, 1);

        let result = run_expr_str::<i32>("1 < 2 and 3 > 2").unwrap();
        assert_eq!(result, 1);

        let result = run_expr_str::<i32>("1 > 2 and 3 > 2").unwrap();
        assert_eq!(result, 0);

        let result = run_expr_str::<i32>("1 < 2 or 3 < 2").unwrap();
        assert_eq!(result, 1);

        let result = run_expr_str::<i32>("1 > 2 or 3 < 2").unwrap();
        assert_eq!(result, 0);
    }

    #[test]
    fn test_module() {
        // multiline string for a module definition
        let module_str = r#"
        fn main() -> i64 {
            return 1 + 2;
        }
        "#;
        let mod = make_module()
    }
}
