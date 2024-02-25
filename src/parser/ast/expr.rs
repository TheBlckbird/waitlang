use thin_vec::ThinVec;

use super::{block::Block, lit::Lit, Span};

#[derive(Debug, PartialEq)]
pub struct Expr {
    pub expr_kind: ExprKind,
    pub span: Span,
}

impl Expr {
    pub fn new(expr_kind: ExprKind, span: Span) -> Self {
        Self { expr_kind, span }
    }
}

#[derive(Debug, PartialEq)]
pub enum ExprKind {
    /// Binary Operation
    ///
    /// ## Example
    /// ```rust
    /// 3 + 5
    /// ```
    BinOp(BinOpKind, Box<Expr>, Box<Expr>),

    /// Unary Operation
    ///
    /// ## Example
    /// ```rust
    /// -5
    /// ```
    UnOp(UnOpKind, Box<Expr>),

    /// Funtion Call
    ///
    /// ## Example
    /// ```rust
    /// print(4)
    /// ```
    FnCall(Ident, Option<ThinVec<Box<Expr>>>),

    /// Method Call
    ///
    /// ## Example
    /// ```rust
    /// a.show(4)
    /// ```
    MethodCall {
        receiver: Box<Expr>,
        ident: Ident,
        args: ThinVec<Box<Expr>>,
    },

    /// Field Access
    ///
    /// ## Example
    /// ```rust
    /// a.number
    /// ```
    FieldAcc(Box<Expr>, Ident),

    /// Literal
    ///
    /// ## Example
    /// ```rust
    /// 4
    /// true
    /// ```
    Lit(Lit),

    /// If
    ///
    /// ## Example
    /// ```rust
    /// if (a == 5) {
    ///     // main block
    /// } else if (a = 4) {
    ///     // else if block
    /// } else {
    ///     // else block
    /// }
    /// ```
    If(Box<Expr>, Box<Block>, Option<Box<Expr>>),

    /// Identifier
    ///
    /// ## Example
    /// ```rust
    /// a
    /// ```
    Ident(Ident),
}

#[derive(Debug, PartialEq)]
pub struct Ident {
    pub name: &'static str,
    pub span: Span,
}

impl Ident {
    pub fn new(name: &'static str, span: Span) -> Self {
        Self { name, span }
    }

    pub fn as_str(&self) -> &str {
        self.name
    }
}

#[derive(Debug, PartialEq)]
pub enum BinOpKind {
    /// Addition
    Add,
    /// Subtraction
    Sub,
    /// Multiplication
    Mul,
    /// Division
    Div,
    /// Modulo
    Mod,
    /// Boolean and
    And,
    /// Boolean or
    Or,
    /// Boolean xor
    Xor,
    /// Boolean equal
    Eq,
    /// Boolean not equal
    Ne,
    /// Boolean less than
    Lt,
    /// Boolean less than or equal
    Le,
    /// Boolean greater than
    Gt,
    /// Boolean greater than or equal
    Ge,
}

#[derive(Debug, PartialEq)]
pub enum UnOpKind {
    /// Boolean not
    Not,
    /// Negation
    Neg,
}
