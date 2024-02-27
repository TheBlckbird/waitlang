use super::expr::{Expr, Ident};
use super::{Span, Type};

#[derive(Debug, PartialEq)]
pub struct Stmt {
    pub stmt_kind: StmtKind,
    pub span: Span,
}

impl Stmt {
    pub fn new(stmt_kind: StmtKind, span: Span) -> Self {
        Self { stmt_kind, span }
    }
}

#[derive(Debug, PartialEq)]
pub enum StmtKind {
    /// Variable binding or declaration
    ///
    /// ## Example
    /// ```javascript
    /// var myvar = 3;
    /// ```
    VarBind {
        type_: Option<Type>,
        identifier: Ident,
        value: Box<Expr>,
    },

    /// Any expression followed by a semicolon
    ///
    /// ## Example
    /// ```rust
    /// print(3);
    /// ```
    Semi(Box<Expr>),

    /// Any expression not followed by a semicolon
    ///
    /// ## Example
    /// ```rust
    /// if (a == 3) {
    ///     // code
    /// }
    Expr(Box<Expr>),
}
