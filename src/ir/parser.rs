use lalrpop_util::lalrpop_mod;
use crate::ir::ast::{Expr, BinOp};
use crate::ir::ast;


lalrpop_mod!(pub ir_grammar, "/ir/ir_grammar.rs");

pub fn parse_expr(input: &str) -> Result<Expr, String> {
    let parser = ir_grammar::ExprParser::new();
    match parser.parse(input) {
        Ok(expr) => Ok(expr),
        Err(e) => Err(format!("{:?}", e)),
    }
}
