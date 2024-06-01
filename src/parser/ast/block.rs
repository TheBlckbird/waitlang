use super::stmt::Stmt;
use super::{Span, Type};
use thin_vec::ThinVec;

#[derive(Debug, PartialEq, Clone)]
pub struct Block {
    pub stmts: Box<[Stmt]>,
    pub span: Span,
    pub type_: Type,
}

impl Block {
    pub fn new(span: Span, type_: Type) -> Self {
        Self {
            stmts: Box::new([]),
            span,
            type_,
        }
    }
}
