use self::stack::Stack;
use crate::parser::ast::Ast;
use std::{cell::RefCell, rc::Rc};

mod block;
mod expr;
mod stack;
mod stmt;

pub struct Interpreter {
    pub stack: Rc<RefCell<Stack>>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            stack: Rc::new(RefCell::new(Stack::new())),
        }
    }

    pub fn run(&self, ast: Ast) -> Result<(), ()> {
        for node in ast.program {
            self.eval_stmt(node, false)?;
        }

        Ok(())
    }
}
