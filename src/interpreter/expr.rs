use super::{
    stack::{StackItem, Variable},
    Interpreter,
};
use crate::parser::ast::{
    expr::{BinOp, Expr, ExprKind, UnOp},
    lit::{Lit, LitKind},
    Type,
};
use std::{ops::Deref, process::exit};

impl Interpreter {
    pub fn eval_expr(&self, expr: &Expr) -> Result<Lit, ()> {
        match &expr.expr_kind {
            ExprKind::Binary(left, bin_op_kind, right) => {
                let exit_condition = match bin_op_kind {
                    BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div | BinOp::Mod => {
                        (left.type_ != right.type_
                            && !(left.type_ == Type::Number && right.type_ == Type::Time)
                            && !(left.type_ == Type::Time && right.type_ == Type::Number))
                            || (left.type_ == Type::Time && right.type_ == Type::Time)
                    }

                    BinOp::And | BinOp::Or | BinOp::Xor => {
                        left.type_ != Type::Bool || right.type_ != Type::Bool
                    }

                    BinOp::EqEq | BinOp::Ne | BinOp::Lt | BinOp::Le | BinOp::Gt | BinOp::Ge => {
                        // No bool
                        (left.type_ != right.type_
                            && !(left.type_ == Type::Number && right.type_ == Type::Time)
                            && !(left.type_ == Type::Time && right.type_ == Type::Number))
                            || (left.type_ == Type::Bool || right.type_ == Type::Bool)
                    }
                };

                if exit_condition {
                    return Err(());
                }

                let left = self.eval_expr(left)?;
                let right = self.eval_expr(right)?;

                match bin_op_kind {
                    BinOp::Add
                    | BinOp::Sub
                    | BinOp::Mul
                    | BinOp::Div
                    | BinOp::Mod
                    | BinOp::EqEq
                    | BinOp::Ne
                    | BinOp::Lt
                    | BinOp::Le
                    | BinOp::Gt
                    | BinOp::Ge => Ok(Lit::new(self.bin_op_num(&left.0, bin_op_kind, &right.0)?)),
                    BinOp::And | BinOp::Or | BinOp::Xor => {
                        Ok(Lit::new(self.bin_op_bool(&left.0, bin_op_kind, &right.0)))
                    }
                }
            }
            ExprKind::Unary(un_op_kind, target_expr) => {
                let target_lit = self.eval_expr(target_expr)?;

                // if (target_lit.type_ == Type::Bool && *un_op_kind != UnOp::Not)
                //     || ((target_lit.type_ == Type::Number || target_lit.type_ == Type::Time)
                //         && *un_op_kind != UnOp::Neg)
                // {
                //     return Err(());
                // }

                let lit_kind = match target_lit.0 {
                    LitKind::Num(num) => LitKind::Num(match un_op_kind {
                        UnOp::Neg => -num,
                        _ => unreachable!(),
                    }),
                    LitKind::Time(time, time_kind) => LitKind::Time(
                        match un_op_kind {
                            UnOp::Neg => -time,
                            _ => unreachable!(),
                        },
                        time_kind,
                    ),
                    LitKind::Bool(bool) => LitKind::Bool(match un_op_kind {
                        UnOp::Not => !bool,
                        _ => unreachable!(),
                    }),
                };

                Ok(Lit::new(lit_kind))
            }
            ExprKind::FnCall(ident, arguments) => {
                let rc = (*self.stack).borrow();
                let function = rc.get_function(ident).ok_or(())?;

                (*self.stack).borrow_mut().push(StackItem::StackMarker);

                for (arg_index, argument) in arguments.deref().iter().enumerate() {
                    if argument.type_ != function.args[arg_index].1 {
                        return Err(());
                    }

                    (*self.stack)
                        .borrow_mut()
                        .push(StackItem::Variable(Variable::new(
                            function.args[arg_index].0,
                            self.eval_expr(argument)?,
                            function.args[arg_index].1,
                        )));
                }

                let maybe_lit = self.eval_block(function.body.clone(), true)?;

                (*self.stack).borrow_mut().pop_scope();

                match maybe_lit {
                    Some(lit) => Ok(lit),
                    None => Err(()),
                }
            }
            ExprKind::MethodCall {
                receiver,
                ident,
                args,
            } => todo!(),
            ExprKind::FieldAcc(_, _) => todo!(),
            ExprKind::Lit(lit) => Ok(*lit),
            ExprKind::Ident(ident) => match (*self.stack).borrow().get_variable(ident) {
                Some(var) => Ok(var.value),
                None => Err(()),
            },
            ExprKind::Grouping(group) => self.eval_expr(group),
        }
    }

    fn bin_op_num(
        &self,
        left: &LitKind,
        bin_op_kind: &BinOp,
        right: &LitKind,
    ) -> Result<LitKind, ()> {
        let mut ret_time_kind = None;

        let left = match left {
            LitKind::Time(num, time_kind) => {
                ret_time_kind = Some(time_kind);
                time_kind.as_ms() * num
            }
            LitKind::Num(num) => *num,
            _ => unreachable!(),
        };

        let right = match right {
            LitKind::Time(num, time_kind) => {
                ret_time_kind = Some(time_kind);
                time_kind.as_ms() * num
            }
            LitKind::Num(num) => *num,
            _ => unreachable!(),
        };

        let num_result = match bin_op_kind {
            BinOp::Add => Some(left + right),
            BinOp::Sub => Some(left - right),
            BinOp::Mul => Some(left * right),
            BinOp::Div => {
                if right == 0. {
                    return Err(());
                }
                Some(left / right)
            }
            BinOp::Mod => {
                if right == 0. {
                    return Err(());
                }
                Some(left % right)
            }
            _ => None,
        };

        let bool_result = match bin_op_kind {
            BinOp::EqEq => Some(left == right),
            BinOp::Ne => Some(left != right),
            BinOp::Lt => Some(left < right),
            BinOp::Le => Some(left <= right),
            BinOp::Gt => Some(left > right),
            BinOp::Ge => Some(left >= right),
            _ => None,
        };

        match ret_time_kind {
            Some(time_kind) => {
                let result = match num_result {
                    Some(num) => num,
                    None => unreachable!(),
                };

                Ok(LitKind::Time(result, *time_kind))
            }
            None => {
                if let Some(num_result) = num_result {
                    return Ok(LitKind::Num(num_result));
                }

                if let Some(bool_result) = bool_result {
                    return Ok(LitKind::Bool(bool_result));
                }

                unreachable!();
            }
        }
    }

    fn bin_op_bool(&self, left: &LitKind, bin_op_kind: &BinOp, right: &LitKind) -> LitKind {
        let left = match left {
            LitKind::Bool(bool) => bool,
            _ => unreachable!(),
        };

        let right = match right {
            LitKind::Bool(bool) => bool,
            _ => unreachable!(),
        };

        let result = match bin_op_kind {
            BinOp::And => *left && *right,
            BinOp::Or => *left || *right,
            BinOp::Xor => left ^ right,
            _ => unreachable!(),
        };

        LitKind::Bool(result)
    }
}
