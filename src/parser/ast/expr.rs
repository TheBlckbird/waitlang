use thin_vec::ThinVec;

use super::{Type, lit::Lit, Span};

#[derive(Debug, PartialEq, Clone)]
pub struct Expr {
    pub expr_kind: ExprKind,
    pub span: Span,
    pub type_: Type,
}

impl Expr {
    pub fn new(expr_kind: ExprKind, span: Span, type_: Type) -> Self {
        Self {
            expr_kind,
            span,
            type_,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ExprKind {
    /// Binary Operation
    ///
    /// ## Example
    /// ```rust
    /// 3 + 5
    /// ```
    Binary(BinOp, Box<Expr>, Box<Expr>),

    /// Unary Operation
    ///
    /// ## Example
    /// ```rust
    /// -5
    /// ```
    Unary(UnOp, Box<Expr>),

    /// Funtion Call
    ///
    /// ## Example
    /// ```rust
    /// print(4)
    /// ```
    FnCall(Ident, Box<[Expr]>),

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

    /// Identifier
    ///
    /// ## Example
    /// ```rust
    /// a
    /// ```
    Ident(Ident),
}

#[derive(Debug, PartialEq, Clone, Copy)]
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

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BinOp {
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

#[derive(Debug, PartialEq, Clone)]
pub enum UnOp {
    /// Boolean not
    Not,
    /// Negation
    Neg,
}
