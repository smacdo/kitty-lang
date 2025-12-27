mod builder;
mod exprs;
mod pretty_printer;

pub use builder::*;
pub use exprs::*;
pub use pretty_printer::*;

use slotmap::SlotMap;

slotmap::new_key_type! { pub struct AstNodeKey; }

#[derive(Debug, PartialEq)]
pub enum AstNode {
    Expr(Expr),
}

pub type AstNodeGraph = SlotMap<AstNodeKey, AstNode>;

// XXX: this is a temporary implementation of the top level, it will be refactored.
pub struct Module {
    pub ast_nodes: AstNodeGraph,
}

/// A literal value is a hard code in the AST, as opposed to an identifier which references a value
/// stored in memory.
#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Null => write!(f, "<null>"),
            Literal::Bool(v) => write!(f, "{}", *v),
            Literal::Int(v) => write!(f, "{}", *v),
            Literal::Float(v) => write!(f, "{}", *v),
            Literal::String(v) => write!(f, "\"{}\"", v),
        }
    }
}
