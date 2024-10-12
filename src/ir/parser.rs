use lalrpop_util::lalrpop_mod;
use crate::ir::ast::{Expr, BinOp, ExprValue};
use crate::ir::ast;


lalrpop_mod!(pub ir_grammar, "/ir/ir_grammar.rs");

pub fn parse_expr(input: &str) -> Result<Box<Expr>, String> {
    let parser = ir_grammar::ExprParser::new();
    match parser.parse(input) {
        Ok(expr) => Ok(expr),
        Err(e) => Err(format!("{:?}", e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arithmetic_ops() {
        let expr = parse_expr("x * 1 + 2 - 3").unwrap();
        // println!("{:#?}", expr);
        assert_eq!(*expr, Expr::new(
            ExprValue::BinOp(
                Box::new(Expr::new(ast::ExprValue::Identifier("x".to_string()))),
                BinOp::Mul,
                Box::new(Expr::new(ExprValue::BinOp(
                    Box::new(Expr::new(ast::ExprValue::BinOp(
                        Box::new(Expr::new(ast::ExprValue::Int(1))),
                        BinOp::Add,
                        Box::new(Expr::new(ast::ExprValue::Int(2)))
                    ))),
                    BinOp::Sub,
                    Box::new(Expr::new(ast::ExprValue::Int(3))),
                )))
            )
        ));
    }
}

