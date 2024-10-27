mod parser {
  pub mod parse;
  pub mod ast;
}

 mod codegen {
   pub mod llvm;
}

fn main() {

    // let expr  = "var1 + var2 + 12";
    // println!("{:#?}", grammar::ExprParser::new().parse(expr).unwrap());
}
