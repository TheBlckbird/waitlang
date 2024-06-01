use super::block::Block;
use super::expr::{Expr, Ident};
use super::{Span, Type};

#[derive(Debug, PartialEq, Clone)]
pub struct Stmt {
    pub stmt_kind: StmtKind,
    pub span: Span,
}

impl Stmt {
    pub fn new(stmt_kind: StmtKind, span: Span) -> Self {
        Self { stmt_kind, span }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum StmtKind {
    /// Variable binding or declaration
    ///
    /// ## Example
    /// ```c
    /// num myvar = 3;
    /// ```
    VarBind {
        type_: Type,
        identifier: Ident,
        value: Box<Expr>,
    },

    /// Any expression followed by a semicolon
    ///
    /// ## Example
    /// ```rust
    /// print(3);
    /// ```
    Expr(Box<Expr>),

    // An if statement
    //
    // ## Example
    // ```rust
    // if (a > 3) {
    //    // true block
    // } else if (a < 3) {
    //   // else if true block
    // } else {
    //  // false block
    // }
    If(Box<Expr>, Box<Block>, Option<Box<Expr>>),

    /// A while loop
    ///
    /// ## Example
    /// ```rust
    /// while (a > 3) {
    ///    // block
    /// }
    /// ```
    While(Box<Expr>, Box<Block>),

    /// A function definition
    ///
    /// ## Example
    /// ```c
    /// num add(num a, num b) {
    ///    return a + b;
    /// }
    /// ```
    FnDef {
        ident: Ident,
        args: Box<[(Ident, Type)]>,
        body: Box<Block>,
        return_type: Type,
    },

    /// A return statement
    ///
    /// ## Example
    /// ```rust
    /// return 3;
    /// ```
    Return(Box<Expr>),
}
