use codegen::llvm::get_basic_value;
use inkwell::{context::Context, execution_engine::JitFunction};

mod parser {
  pub mod parse;
  pub mod ast;
}

mod typing {
  pub mod expr;
  pub mod error;
}

 mod codegen {
   pub mod llvm;
}

fn main() {

  // Parse and type expression
  let mut expr = parser::parse::expr("(1 + 2) * 3").unwrap();
  let empty_hashmap = std::collections::HashMap::new();
  typing::expr::annotate(&empty_hashmap, &mut expr).unwrap();

  // Initialize context and function
  let ctx = Context::create();
  let module = ctx.create_module("main");
  let fn_type = ctx.i32_type().fn_type(&[], false);
  let fn_value = module.add_function("main", fn_type, None);
  let entry_basic_block = ctx.append_basic_block(fn_value, "entry");
  let builder = ctx.create_builder();
  builder.position_at_end(entry_basic_block);

  // Compile expression
  let codegen = codegen::llvm::CodeGen {
    context: &ctx,
    module: &module,
    builder: &builder,
  };
  let compiled_expr = codegen::llvm::expr(&empty_hashmap, &codegen, &expr).unwrap();
  let compiled_expr_val = get_basic_value(&compiled_expr);
  let ret = builder.build_return(Some(&*compiled_expr_val)).unwrap(); // the &*: remove the box,
                                                                      // then pass as reference.
  // Run
  let ee = module.create_jit_execution_engine(inkwell::OptimizationLevel::None).unwrap(); 
  let result = unsafe { 
    type MainFn = unsafe extern "C" fn() -> i32;
    let main: JitFunction<MainFn> = ee.get_function("main").unwrap();
    main.call()
  };

  println!("{:#?}", result);
}
