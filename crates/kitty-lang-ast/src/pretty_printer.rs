use crate::{AstNodeGraph, ExprNodeKey, ExprVisitor, visit_expr};

pub fn pretty_print_expr(ast: &AstNodeGraph, expr_nk: ExprNodeKey) -> String {
    let mut visitor = PrettyPrinterVisitor { ast };

    visit_expr(&mut visitor, ast, expr_nk)
}

pub struct PrettyPrinterVisitor<'a> {
    ast: &'a AstNodeGraph,
}

impl ExprVisitor<String> for PrettyPrinterVisitor<'_> {
    fn visit_binary(&mut self, node: &crate::BinaryExpr) -> String {
        format!(
            "({} {} {})",
            node.op,
            pretty_print_expr(self.ast, node.left),
            pretty_print_expr(self.ast, node.right),
        )
    }

    fn visit_grouping(&mut self, node: &crate::GroupingExpr) -> String {
        format!("(group {})", pretty_print_expr(self.ast, node.expr))
    }

    fn visit_literal(&mut self, node: &crate::LiteralExpr) -> String {
        format!("{}", node.literal)
    }

    fn visit_unary(&mut self, node: &crate::UnaryExpr) -> String {
        format!("({} {})", node.op, pretty_print_expr(self.ast, node.expr),)
    }
}

#[cfg(test)]
mod tests {
    use crate::AstExprConstructor;

    use super::*;

    #[test]
    fn pretty_print_example() {
        let mut ast: AstNodeGraph = Default::default();
        let mut builder = AstExprConstructor::new(&mut ast);
        let expr_nk = builder.binary(
            crate::BinaryOp::Mul,
            |b| {
                b.unary(crate::UnaryOp::Negate, |b| {
                    b.literal(crate::Literal::Int(123))
                })
            },
            |b| b.grouping(|b| b.literal(crate::Literal::Float(45.67))),
        );

        assert_eq!(
            pretty_print_expr(&ast, expr_nk),
            "(* (- 123) (group 45.67))".to_string()
        );
    }
}
