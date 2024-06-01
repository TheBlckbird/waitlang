use crate::parser::ast::{block::Block, expr::Ident, lit::Lit, Type};

pub struct Stack {
    pub stack: Vec<StackItem>,
}

#[derive(Debug, PartialEq)]
pub struct Variable {
    pub ident: Ident,
    pub value: Lit,
    pub type_: Type,
}

impl Variable {
    pub fn new(ident: Ident, value: Lit, type_: Type) -> Self {
        Self {
            ident,
            value,
            type_,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Function {
    pub ident: Ident,
    pub args: Box<[(Ident, Type)]>,
    pub body: Box<Block>,
    pub return_type: Type,
}

impl Function {
    pub fn new(
        ident: Ident,
        args: Box<[(Ident, Type)]>,
        body: Box<Block>,
        return_type: Type,
    ) -> Self {
        Self {
            ident,
            args,
            body,
            return_type,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum StackItem {
    Variable(Variable),
    Function(Function),
    StackMarker,
}

impl Stack {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn push(&mut self, item: StackItem) {
        self.stack.push(item);
    }

    pub fn pop(&mut self) -> Option<StackItem> {
        self.stack.pop()
    }

    pub fn pop_scope(&mut self) {
        loop {
            match self.stack.pop() {
                Some(StackItem::StackMarker) => break,
                Some(_) => {}
                None => break,
            }
        }
    }

    pub fn get_variable(&self, ident: &Ident) -> Option<&Variable> {
        for item in self.stack.iter().rev() {
            if let StackItem::Variable(var) = item {
                if var.ident == *ident {
                    return Some(var);
                }
            }
        }

        None
    }

    pub fn get_function(&self, ident: &Ident) -> Option<&Function> {
        for item in self.stack.iter().rev() {
            if let StackItem::Function(func) = item {
                if func.ident == *ident {
                    return Some(func);
                }
            }
        }

        None
    }
}
