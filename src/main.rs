use lalrpop_util::lalrpop_mod;
mod ast;

lalrpop_mod!(pub grammar);

fn main() {
    let fndef  = "fn (var1, var2) {
        somefun = fn (a, b) a + b;
        another_fun = fn (b) {
            b + 1;
        }
        var1 + var2;
    }";
    println!("{:#?}", grammar::FunDefBlockParser::new().parse(fndef).unwrap());

    // let expr  = "var1 + var2 + 12";
    // println!("{:#?}", grammar::ExprParser::new().parse(expr).unwrap());
}
