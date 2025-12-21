use crate::{AstNodeGraph, BinaryOp, Expr, ExprNodeKey, Literal, UnaryOp};

pub struct AstBuilder {}

impl AstBuilder {}

pub struct ExprAstBuilder<'a> {
    ast: &'a mut AstNodeGraph,
}

impl<'a> ExprAstBuilder<'a> {
    pub fn new(ast: &'a mut AstNodeGraph) -> Self {
        Self { ast }
    }

    pub fn binary<F1, F2>(&mut self, op: BinaryOp, left_f: F1, right_f: F2) -> ExprNodeKey
    where
        F1: Fn(&mut ExprAstBuilder) -> ExprNodeKey,
        F2: Fn(&mut ExprAstBuilder) -> ExprNodeKey,
    {
        let left_nk = left_f(self);
        let right_nk = right_f(self);

        let nk = self
            .ast
            .insert(crate::AstNode::Expr(Expr::Binary(crate::BinaryExpr {
                left: left_nk,
                right: right_nk,
                op,
            })));
        ExprNodeKey::new(nk)
    }

    pub fn grouping<F>(&mut self, f: F) -> ExprNodeKey
    where
        F: Fn(&mut ExprAstBuilder) -> ExprNodeKey,
    {
        let group_nk = f(self);
        let nk = self
            .ast
            .insert(crate::AstNode::Expr(Expr::Grouping(crate::GroupingExpr {
                expr: group_nk,
            })));
        ExprNodeKey::new(nk)
    }

    pub fn literal(&mut self, literal: Literal) -> ExprNodeKey {
        let nk = self
            .ast
            .insert(crate::AstNode::Expr(Expr::Literal(crate::LiteralExpr {
                literal,
            })));
        ExprNodeKey::new(nk)
    }

    pub fn unary<F>(&mut self, op: UnaryOp, expr_f: F) -> ExprNodeKey
    where
        F: Fn(&mut ExprAstBuilder) -> ExprNodeKey,
    {
        let expr_nk = expr_f(self);

        let nk = self
            .ast
            .insert(crate::AstNode::Expr(Expr::Unary(crate::UnaryExpr {
                expr: expr_nk,
                op,
            })));
        ExprNodeKey::new(nk)
    }
}
