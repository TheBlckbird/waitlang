use crate::parser::ast::{lit::TimeKind, Span};
use std::fmt::Display;

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

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self {
            TokenKind::Ident(ident) => ident.clone(),
            TokenKind::Num(num) => num.to_string(),
            TokenKind::Time(num, time_kind) => format!(
                "{num}{}",
                match time_kind {
                    TimeKind::Ms => "ms",
                    TimeKind::Sec => "s",
                    TimeKind::Min => "min",
                    TimeKind::Hour => "h",
                    TimeKind::Day => "d",
                    TimeKind::Week => "w",
                    TimeKind::Year => "y",
                }
            ),
            TokenKind::Bool(bool) => bool.to_string(),
            _ => String::from(match self {
                TokenKind::Add => "+",
                TokenKind::Sub => "-",
                TokenKind::Mul => "*",
                TokenKind::Div => "/",
                TokenKind::Mod => "%",
                TokenKind::And => "&&",
                TokenKind::Or => "||",
                TokenKind::Xor => "^^",
                TokenKind::EqEq => "==",
                TokenKind::Ne => "!=",
                TokenKind::Lt => "<",
                TokenKind::Le => "<=",
                TokenKind::Gt => ">",
                TokenKind::Ge => ">=",
                TokenKind::Not => "!",
                TokenKind::OpenBracket => "(",
                TokenKind::CloseBracket => ")",
                TokenKind::OpenCurlBracket => "{",
                TokenKind::CloseCurlBracket => "}",
                TokenKind::Col => ":",
                TokenKind::Semi => ";",
                TokenKind::Arrow => "->",
                TokenKind::Eq => "=",
                TokenKind::Func => "func",
                TokenKind::Eof => "",
                _ => unreachable!(),
            }),
        };

        write!(f, "{out}")
    }
}
