use super::stmt::Stmt;
use super::{Span, Type};
use thin_vec::ThinVec;

#[derive(Debug, PartialEq)]
pub struct Block {
    pub stmts: ThinVec<Stmt>,
    pub span: Span,
    pub type_: Type,
}

impl Block {
    pub fn new(span: Span, type_: Type) -> Self {
        Self {
            stmts: ThinVec::new(),
            span,
            type_,
        }
    }
}
