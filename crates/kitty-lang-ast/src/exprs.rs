use crate::{AstNode, AstNodeGraph, AstNodeKey, Literal};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ExprNodeKey(AstNodeKey);

impl ExprNodeKey {
    pub fn new(nk: AstNodeKey) -> Self {
        Self(nk)
    }
}

impl From<ExprNodeKey> for AstNodeKey {
    fn from(value: ExprNodeKey) -> Self {
        value.0
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum BinaryOp {
    Add,
    Mul,
}

impl std::fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryOp::Add => write!(f, "+"),
            BinaryOp::Mul => write!(f, "*"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum UnaryOp {
    Negate,
}

impl std::fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnaryOp::Negate => write!(f, "-"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct BinaryExpr {
    pub left: ExprNodeKey,
    pub right: ExprNodeKey,
    pub op: BinaryOp,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GroupingExpr {
    pub expr: ExprNodeKey,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LiteralExpr {
    pub literal: Literal,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UnaryExpr {
    pub expr: ExprNodeKey,
    pub op: UnaryOp,
}

/// All the expression expression forms in the language. Each expression node in kitty-lang's AST
/// will appear here.
#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Binary(BinaryExpr),
    Grouping(GroupingExpr),
    Literal(LiteralExpr),
    Unary(UnaryExpr),
}

/// Visitor strategy type trait for expr AST nodes. Use `visit_expr()` on a type implementing
/// `ExprVisitor<T>` to dispatch each AST node type to the appropriate visitor method.
///
/// `T` represents the value returned from a visitor.
pub trait ExprVisitor<T> {
    fn visit_binary(&mut self, binary: &BinaryExpr) -> T;
    fn visit_grouping(&mut self, binary: &GroupingExpr) -> T;
    fn visit_literal(&mut self, binary: &LiteralExpr) -> T;
    fn visit_unary(&mut self, binary: &UnaryExpr) -> T;
}

/// Visitor strategy type trait for expr AST nodes. Use `visit_expr()` on a type implementing
/// `ExprVisitor<T>` to dispatch each AST node type to the appropriate visitor method.
///
/// `T` represents the value returned from a visitor.
pub trait MutableExprVisitor<T> {
    fn visit_binary(&mut self, binary: &mut BinaryExpr) -> T;
    fn visit_grouping(&mut self, binary: &mut GroupingExpr) -> T;
    fn visit_literal(&mut self, binary: &mut LiteralExpr) -> T;
    fn visit_unary(&mut self, binary: &mut UnaryExpr) -> T;
}

/// Calls the correct visitor method for the given expression AST node.
pub fn visit_expr<U, T>(visitor: &mut T, ast_nodes: &AstNodeGraph, expr_nk: ExprNodeKey) -> U
where
    T: ExprVisitor<U>,
{
    let node = ast_nodes
        .get(expr_nk.into())
        .expect("expr ast node key must exist in this AST graph");

    match node {
        AstNode::Expr(expr) => match expr {
            Expr::Binary(node) => visitor.visit_binary(node),
            Expr::Grouping(node) => visitor.visit_grouping(node),
            Expr::Literal(node) => visitor.visit_literal(node),
            Expr::Unary(node) => visitor.visit_unary(node),
        },
    }
}

/// Calls the correct visitor method for the given expression AST node.
pub fn visit_expr_mut<U, T>(
    visitor: &mut T,
    ast_nodes: &mut AstNodeGraph,
    expr_nk: ExprNodeKey,
) -> U
where
    T: MutableExprVisitor<U>,
{
    let node = ast_nodes
        .get_mut(expr_nk.into())
        .expect("expr ast node key must exist in this AST graph");

    match node {
        AstNode::Expr(expr) => match expr {
            Expr::Binary(node) => visitor.visit_binary(node),
            Expr::Grouping(node) => visitor.visit_grouping(node),
            Expr::Literal(node) => visitor.visit_literal(node),
            Expr::Unary(node) => visitor.visit_unary(node),
        },
    }
}
