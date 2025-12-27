use std::{collections::HashMap, mem::Discriminant};

use kitty_lang_ast::{AstExprConstructor, AstNodeGraph, ExprNodeKey};

use crate::{parser::ParseError, scanner::Scanner, tokens::Token};

pub struct Foo {}

pub struct PrattGrammar {
    pub ops: HashMap<Discriminant<Token>, Foo>,
}

impl PrattGrammar {
    pub fn new() -> Self {
        Self {
            ops: HashMap::from([
                (std::mem::discriminant(&Token::Plus), Foo {}),
                (std::mem::discriminant(&Token::Star), Foo {}),
            ]),
        }
    }

    fn is_op_token(&self, op: &Token) -> bool {
        self.ops.contains_key(&std::mem::discriminant(op))
    }
}

pub fn parse_expr(
    grammar: &PrattGrammar,
    scanner: &mut Scanner,
    ast: &mut AstNodeGraph,
) -> Result<ExprNodeKey, ParseError> {
    let mut expr_constructor = AstExprConstructor::new(ast);
    parse_expr_with_precedence(grammar, scanner, &mut expr_constructor)
}

pub fn parse_expr_with_precedence(
    grammar: &PrattGrammar,
    scanner: &mut Scanner,
    expr_constructor: &mut AstExprConstructor,
) -> Result<ExprNodeKey, ParseError> {
    let lhs_nk = match scanner.next() {
        Some(lexeme) if lexeme.token == Token::Int => {
            match lexeme.token {
                Token::Int()
            }
            expr_constructor.literal(literal)
        }
        lexeme => panic!("BAD LEXEME: {:?} (IMPL ERROR HANDLING)", lexeme),
    };

    loop {
        let _operator = match scanner.next() {
            None => break, // end of stream
            Some(lexeme) if grammar.is_op_token(&lexeme.token) => lexeme.token,
            lexeme => panic!("BAD LEXEME: {:?} - IMPL ERR HANDLING", lexeme),
        };

        todo!("implement me! -- parser.rs:25");
    }

    Ok(lhs)
}

#[cfg(test)]
mod tests {
    use kitty_lang_ast::{AstNode, Expr, Literal, LiteralExpr};

    use super::*;

    fn parse(source: &str) -> Result<(AstNodeGraph, ExprNodeKey), ParseError> {
        let mut ast: AstNodeGraph = Default::default();
        let mut scanner = Scanner::new(source);
        let expr_nk = parse_expr_with_precedence(&PrattGrammar::new(), &mut scanner, &mut ast)?;
        Ok((ast, expr_nk))
    }

    #[test]
    fn parse_literal_number() {
        let (ast, literal_expr_nk) = parse("1024").unwrap();
        assert_eq!(
            ast[literal_expr_nk.into()],
            AstNode::Expr(Expr::Literal(LiteralExpr {
                literal: Literal::Int(1024)
            }))
        );
    }
}
