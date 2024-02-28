use crate::parser::ast::{
    expr::{BinOpKind, Expr, ExprKind},
    lit::{Lit, LitKind},
    Type,
};

use super::Interpreter;

impl Interpreter {
    pub fn eval_expr(&mut self, expr: Box<Expr>) -> Result<Lit, ()> {
        match expr.expr_kind {
            ExprKind::BinOp(bin_op_kind, left, right) => {
                let exit_condition = match bin_op_kind {
                    BinOpKind::Add
                    | BinOpKind::Sub
                    | BinOpKind::Mul
                    | BinOpKind::Div
                    | BinOpKind::Mod => {
                        (left.type_ != right.type_
                            && !(left.type_ == Type::Number && right.type_ == Type::Time)
                            && !(left.type_ == Type::Time && right.type_ == Type::Number))
                            || (left.type_ == Type::Time && right.type_ == Type::Time)
                    }

                    BinOpKind::And | BinOpKind::Or | BinOpKind::Xor => {
                        left.type_ != Type::Bool || right.type_ != Type::Bool
                    }

                    BinOpKind::Eq
                    | BinOpKind::Ne
                    | BinOpKind::Lt
                    | BinOpKind::Le
                    | BinOpKind::Gt
                    | BinOpKind::Ge => {
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
                    BinOpKind::Add
                    | BinOpKind::Sub
                    | BinOpKind::Mul
                    | BinOpKind::Div
                    | BinOpKind::Mod
                    | BinOpKind::Eq
                    | BinOpKind::Ne
                    | BinOpKind::Lt
                    | BinOpKind::Le
                    | BinOpKind::Gt
                    | BinOpKind::Ge => Ok(Lit::new(
                        self.bin_op_num(bin_op_kind, left.lit_kind, right.lit_kind)?,
                        expr.span,
                        expr.type_,
                    )),
                    BinOpKind::And | BinOpKind::Or | BinOpKind::Xor => Ok(Lit::new(
                        self.bin_op_bool(bin_op_kind, left.lit_kind, right.lit_kind),
                        expr.span,
                        expr.type_,
                    )),
                }
            }
            ExprKind::UnOp(_, _) => todo!(),
            ExprKind::FnCall(_, _) => todo!(),
            ExprKind::MethodCall {
                receiver,
                ident,
                args,
            } => todo!(),
            ExprKind::FieldAcc(_, _) => todo!(),
            ExprKind::Lit(lit) => Ok(lit),
            ExprKind::If(_, _, _) => todo!(),
            ExprKind::Ident(_) => todo!(),
        }
    }

    fn bin_op_num(
        &self,
        bin_op_kind: BinOpKind,
        left: LitKind,
        right: LitKind,
    ) -> Result<LitKind, ()> {
        let mut ret_time_kind = None;

        let left = match left {
            LitKind::Time(num, time_kind) => {
                ret_time_kind = Some(time_kind);
                time_kind.as_ms() * num
            }
            LitKind::Num(num) => num,
            _ => unreachable!(),
        };

        let right = match right {
            LitKind::Time(num, time_kind) => {
                ret_time_kind = Some(time_kind);
                time_kind.as_ms() * num
            }
            LitKind::Num(num) => num,
            _ => unreachable!(),
        };

        let num_result = match bin_op_kind {
            BinOpKind::Add => Some(left + right),
            BinOpKind::Sub => Some(left - right),
            BinOpKind::Mul => Some(left * right),
            BinOpKind::Div => {
                if right == 0. {
                    return Err(());
                }
                Some(left / right)
            }
            BinOpKind::Mod => {
                if right == 0. {
                    return Err(());
                }
                Some(left % right)
            }
            _ => None,
        };

        let bool_result = match bin_op_kind {
            BinOpKind::Eq => Some(left == right),
            BinOpKind::Ne => Some(left != right),
            BinOpKind::Lt => Some(left < right),
            BinOpKind::Le => Some(left <= right),
            BinOpKind::Gt => Some(left > right),
            BinOpKind::Ge => Some(left >= right),
            _ => None,
        };

        match ret_time_kind {
            Some(time_kind) => {
                let result = match num_result {
                    Some(num) => num,
                    None => unreachable!(),
                };

                Ok(LitKind::Time(result, time_kind))
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

    fn bin_op_bool(&self, bin_op_kind: BinOpKind, left: LitKind, right: LitKind) -> LitKind {
        let left = match left {
            LitKind::Bool(bool) => bool,
            _ => unreachable!(),
        };

        let right = match right {
            LitKind::Bool(bool) => bool,
            _ => unreachable!(),
        };

        let result = match bin_op_kind {
            BinOpKind::And => left && right,
            BinOpKind::Or => left || right,
            BinOpKind::Xor => left ^ right,
            _ => unreachable!(),
        };

        LitKind::Bool(result)
    }
}
