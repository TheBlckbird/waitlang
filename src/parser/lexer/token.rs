use crate::parser::ast::{lit::TimeKind, Span};

#[derive(Debug)]
pub struct Token {
    pub token_kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn new(token_kind: TokenKind, span: Span) -> Self {
        Self { token_kind, span }
    }
}

#[derive(Debug)]
pub enum TokenKind {
    /// Addition x
    Add,
    /// Subtraction x
    Sub,
    /// Multiplication x
    Mul,
    /// Division x
    Div,
    /// Modulo x
    Mod,
    /// Boolean and x
    And,
    /// Boolean or x
    Or,
    /// Boolean xor x
    Xor,
    /// Boolean equal x
    Eq,
    /// Boolean not equal
    Ne,
    /// Boolean less than x
    Lt,
    /// Boolean less than or equal x
    Le,
    /// Boolean greater than x
    Gt,
    /// Boolean greater than or equal x
    Ge,
    /// Boolean not x
    Not,
    /// Identifier x
    Ident(String),
    /// Opening Bracket x
    OpenBracket,
    /// Closing Bracket x
    CloseBracket,
    /// Opening curly Bracket x
    OpenCurlBracket,
    /// Closing Curly Bracket x
    CloseCurlBracket,
    /// Colon x
    Col,
    /// Semicolon x
    Semi,
    /// Arrow x
    Arrow,
    /// Initialization equal x
    Is,
    /// Number x
    Num(f32),
    /// Time in different units x
    Time(f32, TimeKind),
    /// Boolean x
    Bool(bool),
    /// function keyword
    Func,
    /// End of File x
    Eof,
}
