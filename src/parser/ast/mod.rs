use std::fmt::Display;

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
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

impl From<(i32, i32)> for Span {
    fn from(value: (i32, i32)) -> Self {
        Self {
            start: value.0 as usize,
            end: value.1 as usize,
        }
    }
}

impl From<i32> for Span {
    fn from(value: i32) -> Self {
        Self {
            start: value as usize,
            end: value as usize,
        }
    }
}

impl From<(usize, usize)> for Span {
    fn from(value: (usize, usize)) -> Self {
        Self {
            start: value.0,
            end: value.1,
        }
    }
}

impl From<usize> for Span {
    fn from(value: usize) -> Self {
        Self {
            start: value,
            end: value,
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
