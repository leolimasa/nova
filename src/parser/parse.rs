use lalrpop_util::lalrpop_mod;
use crate::parser::ast::*;
use std::collections::HashMap;

lalrpop_mod!(pub grammar, "/parser/grammar.rs");

pub fn expr(input: &str) -> Result<Expr, String> {
    let parser = grammar::ExprParser::new();
    match parser.parse(input) {
        Ok(expr) => Ok(expr),
        Err(e) => Err(format!("{:?}", e)),
    }
}

