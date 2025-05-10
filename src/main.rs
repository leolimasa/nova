//use codegen::llvm::get_basic_value;
//use inkwell::{context::Context, execution_engine::JitFunction};

mod parser {
  pub mod parse;
  pub mod ast;
}

mod typing {
  pub mod expr;
  pub mod error;
}

/* mod codegen {
   pub mod llvm;
} */

fn main() {

  // Parse and type expression
  let mut expr = parser::parse::expr("(1 + 2) * 3").unwrap();
  let empty_hashmap = std::collections::HashMap::new();
  typing::expr::annotate(&empty_hashmap, &mut expr).unwrap();
  println!("{:?}", expr);
  
}
