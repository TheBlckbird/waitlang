use crate::parser::ast::{
    lit::{Lit, LitKind},
    stmt::{Stmt, StmtKind},
};

use super::{
    stack::{Function, StackItem, Variable},
    Interpreter,
};
impl Interpreter {
    pub fn eval_stmt(&self, stmt: Stmt, in_function: bool) -> Result<Option<Lit>, ()> {
        match stmt.stmt_kind {
            StmtKind::VarBind {
                type_,
                identifier,
                value,
            } => {
                let value = self.eval_expr(&*value)?;
                self.stack.borrow_mut().push(StackItem::Variable(Variable {
                    ident: identifier,
                    value,
                    type_,
                }));
            }
            StmtKind::FnDef {
                ident,
                args,
                body,
                return_type,
            } => {
                self.stack.borrow_mut().push(StackItem::Function(Function {
                    ident,
                    args,
                    body,
                    return_type,
                }));
            }
            StmtKind::Expr(expr) => {
                self.eval_expr(&*expr)?;
            }
            StmtKind::If(condition, true_block, else_expr) => {
                let condition = if let LitKind::Bool(bool) = self.eval_expr(&condition)?.lit_kind {
                    bool
                } else {
                    return Err(());
                };

                if condition {
                    self.eval_block(true_block, in_function)?;
                } else if let Some(else_expr) = else_expr {
                    self.eval_expr(&*else_expr)?;
                }
            }
            StmtKind::While(condition_expr, block) => {
                let mut condition =
                    match self.eval_expr(&condition_expr)?.lit_kind {
                        LitKind::Bool(bool) => bool,
                        _ => return Err(()),
                    };

                while condition {
                    self.eval_block(block.clone(), in_function)?;

                    condition = match self.eval_expr(&condition_expr)?.lit_kind {
                        LitKind::Bool(bool) => bool,
                        _ => return Err(()),
                    };
                }
            }
            StmtKind::Return(expr) => {
                if !in_function {
                    return Err(());
                }

                return Ok(Some(self.eval_expr(&expr)?))
            }
        }

        Ok(None)
    }
}
