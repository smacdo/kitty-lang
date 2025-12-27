use kitty_lang_ast::{AstNodeGraph, ExprNodeKey};

use crate::{pratt, scanner::Scanner};

#[derive(Debug)]
pub struct ParseError {}

pub fn parse_expr(scanner: &mut Scanner) -> Result<(AstNodeGraph, ExprNodeKey), ParseError> {
    let mut ast: AstNodeGraph = Default::default();
    let expr_nk =
        pratt::parse_expr_with_precedence(&pratt::PrattGrammar::new(), scanner, &mut ast)?;

    Ok((ast, expr_nk))
}
