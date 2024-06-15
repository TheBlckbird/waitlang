use crate::parser::ast::{lit::TimeKind, Span};

#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn new(token_kind: TokenKind, span: Span) -> Self {
        Self {
            kind: token_kind,
            span,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
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
    /// Boolean or
    Xor,
    /// Boolean equal
    EqEq,
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
    /// Boolean not
    Not,
    /// Identifier
    Ident(String),
    /// Opening Bracket
    OpenBracket,
    /// Closing Bracket
    CloseBracket,
    /// Opening curly Bracket
    OpenCurlBracket,
    /// Closing Curly Bracket
    CloseCurlBracket,
    /// Colon
    Col,
    /// Semicolon
    Semi,
    /// Arrow
    Arrow,
    /// Initialization equal
    Eq,
    /// Number
    Num(f32),
    /// Time in different units
    Time(f32, TimeKind),
    /// Boolean
    Bool(bool),
    /// function keyword
    Func,
    /// End of File
    Eof,
}
