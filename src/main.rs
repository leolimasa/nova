use lalrpop_util::lalrpop_mod;
mod ast;
mod ir {
  pub mod ast;
  pub mod parser;
}
mod compiler {
  pub mod wasm;
}

lalrpop_mod!(pub grammar);

fn main() {
    let fndef  = "fn (var1, var2) {
        apply_me(a, b) + 3 + hello.world();
    }";
    println!("{:#?}", grammar::FunDefBlockParser::new().parse(fndef).unwrap());

    // let expr  = "var1 + var2 + 12";
    // println!("{:#?}", grammar::ExprParser::new().parse(expr).unwrap());
}
