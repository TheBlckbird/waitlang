use crate::parser::ast::{lit::Lit, stmt::StmtKind, Ast, Span};

mod expr;

#[derive(Debug)]
pub struct Variable {
    ident: String,
    value: Lit,
}

pub struct Interpreter {
    variables: Vec<Variable>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            variables: Vec::new(),
        }
    }

    pub fn run(&mut self, ast: Ast) -> Result<(), ()> {
        for node in ast.program {
            match node.stmt_kind {
                StmtKind::VarBind {
                    type_,
                    identifier,
                    value,
                } => todo!(),
                StmtKind::Semi(_) => todo!(),
                StmtKind::Expr(_) => todo!(),
            }
        }

        Ok(())
    }
}
