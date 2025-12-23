use kitty_lang_ast::{AstNodeGraph, ExprNodeKey};

use crate::scanner::Scanner;

pub struct ParseError {}

pub fn parse_expr(scanner: &mut Scanner) -> Result<(AstNodeGraph, ExprNodeKey), ParseError> {
    let mut ast: AstNodeGraph = Default::default();
    todo!("implement me! -- parser.rs:13");
    //let expr_nk = equality(scanner, &mut ast)?;
    //Ok((ast, expr_nk))
}

fn equality(scanner: &mut Scanner, ast: &mut AstNodeGraph) -> Result<ExprNodeKey, ParseError> {
    todo!("implement me! -- parser.rs:13");
}

fn comparison(scanner: &mut Scanner, ast: &mut AstNodeGraph) -> Result<ExprNodeKey, ParseError> {
    todo!("implement me! -- parser.rs:19");
}
