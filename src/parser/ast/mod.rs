use self::stmt::Stmt;

pub mod block;
pub mod expr;
pub mod lit;
pub mod stmt;

#[derive(Debug)]
pub struct Ast {
    pub program: Vec<Stmt>,
}

impl Ast {
    pub fn new() -> Self {
        Self {
            program: Vec::new(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Span {
    start: i32,
    end: i32,
}

impl Span {
    pub fn new(start: i32, end: i32) -> Self {
        Self { start, end }
    }
}

impl From<(i32, i32)> for Span {
    fn from(value: (i32, i32)) -> Self {
        Self {
            start: value.0,
            end: value.1,
        }
    }
}
