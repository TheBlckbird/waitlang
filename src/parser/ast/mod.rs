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

#[derive(Debug, PartialEq, Clone, Copy)]
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

impl From<i32> for Span {
    fn from(value: i32) -> Self {
        Self {
            start: value,
            end: value,
        }
    }
}

impl From<(usize, usize)> for Span {
    fn from(value: (usize, usize)) -> Self {
        Self {
            start: value.0 as i32,
            end: value.1 as i32,
        }
    }
}

impl From<usize> for Span {
    fn from(value: usize) -> Self {
        Self {
            start: value as i32,
            end: value as i32,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Type {
    Time,
    Number,
    Bool,
    User,
    Unit,
}
