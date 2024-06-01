use super::Interpreter;
use crate::parser::ast::{block::Block, lit::Lit};

impl Interpreter {
    pub fn eval_block(&self, block: Box<Block>, in_function: bool) -> Result<Option<Lit>, ()> {
        let block = *block;

        for stmt in block.stmts.iter() {
            let stmt_value = self.eval_stmt(stmt.clone(), in_function)?;

            if let Some(stmt_value) = stmt_value {
                return Ok(Some(stmt_value));
            }
        }

        if in_function {
            Err(())
        } else {
            Ok(None)
        }
    }
}
