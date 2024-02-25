use super::stmt::Stmt;
use super::Span;
use thin_vec::ThinVec;

#[derive(Debug, PartialEq)]
pub struct Block {
    pub stmts: ThinVec<Stmt>,
    pub span: Span,
}

impl Block {
    pub fn new(span: Span) -> Self {
        Self {
            stmts: ThinVec::new(),
            span,
        }
    }
}
