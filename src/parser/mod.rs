use self::ast::Ast;
use ast::{
    expr::{BinOp, Expr, ExprKind, UnOp},
    lit::{Lit, LitKind},
    stmt::{Stmt, StmtKind},
    Span, Type,
};
use lexer::{
    lexer,
    token::{Token, TokenKind},
};
use parse_error::ParseError;

pub mod ast;
pub mod lexer;
mod parse_error;

pub fn parse(code: &str) -> Option<Ast> {
    let tokens = lexer(code).unwrap();

    let mut parser = Parser::new(tokens);
    parser.parse().ok()
}

struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Ast, ParseError> {
        let mut ast = Ast::new();

        ast.program.push(Stmt::new(
            StmtKind::Expr(Box::new(self.expression()?)),
            Span::new(0, 0),
        ));

        Ok(ast)
    }

    fn r#match(&mut self, token_kinds: Vec<TokenKind>) -> bool {
        for kind in token_kinds {
            if self.check(kind) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn check(&self, token_kind: TokenKind) -> bool {
        if self.is_at_end() {
            return false;
        }

        // self.peek().kind == token_kind
        self.peek().kind == token_kind
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        return self.previous();
    }

    fn is_at_end(&self) -> bool {
        self.peek().kind == TokenKind::Eof
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn token_to_bin_op(&self, token: &Token) -> BinOp {
        match token.kind {
            TokenKind::Add => BinOp::Add,
            TokenKind::Sub => BinOp::Sub,
            TokenKind::Mul => BinOp::Mul,
            TokenKind::Div => BinOp::Div,
            TokenKind::Mod => BinOp::Mod,
            TokenKind::And => BinOp::And,
            TokenKind::Or => BinOp::Or,
            TokenKind::Xor => BinOp::Xor,
            TokenKind::EqEq => BinOp::EqEq,
            TokenKind::Ne => BinOp::Ne,
            TokenKind::Lt => BinOp::Lt,
            TokenKind::Le => BinOp::Le,
            TokenKind::Gt => BinOp::Gt,
            TokenKind::Ge => BinOp::Ge,
            _ => unreachable!(),
        }
    }

    fn token_to_un_op(&self, token: &Token) -> UnOp {
        match token.kind {
            TokenKind::Not => UnOp::Not,
            TokenKind::Sub => UnOp::Neg,
            _ => unreachable!(),
        }
    }

    pub fn expression(&mut self) -> Result<Expr, ParseError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.comparison()?;

        while self.r#match(vec![TokenKind::EqEq, TokenKind::Ne]) {
            let operator = self.token_to_bin_op(self.previous());

            let right = self.comparison()?;
            expr = Expr::new(
                ExprKind::Binary(Box::new(expr), operator, Box::new(right)),
                Span::new(0, 0),
                Type::Unit,
            );
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.term()?;

        while self.r#match(vec![
            TokenKind::Gt,
            TokenKind::Ge,
            TokenKind::Le,
            TokenKind::Lt,
        ]) {
            let operator = self.token_to_bin_op(self.previous());
            let right = self.term()?;
            expr = Expr::new(
                ExprKind::Binary(Box::new(expr), operator, Box::new(right)),
                Span::new(0, 0),
                Type::Unit,
            )
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor()?;

        while self.r#match(vec![TokenKind::Sub, TokenKind::Add]) {
            let operator = self.token_to_bin_op(self.previous());
            let right = self.factor()?;
            expr = Expr::new(
                ExprKind::Binary(Box::new(expr), operator, Box::new(right)),
                Span::new(0, 0),
                Type::Unit,
            );
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary()?;

        while self.r#match(vec![TokenKind::Mul, TokenKind::Div, TokenKind::Mod]) {
            let operator = self.token_to_bin_op(self.previous());
            let right = self.unary()?;
            expr = Expr::new(
                ExprKind::Binary(Box::new(expr), operator, Box::new(right)),
                Span::new(0, 0),
                Type::Unit,
            );
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.r#match(vec![TokenKind::Not, TokenKind::Sub]) {
            let operator = self.token_to_un_op(self.previous());
            let right = self.unary()?;
            return Ok(Expr::new(
                ExprKind::Unary(operator, Box::new(right)),
                Span::new(0, 0),
                Type::Unit,
            ));
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        let expr = match self.peek().kind {
            TokenKind::Bool(v) => Some(Expr::new(
                ExprKind::Lit(Lit::new(LitKind::Bool(v))),
                Span::new(0, 0),
                Type::Unit,
            )),
            TokenKind::Num(v) => Some(Expr::new(
                ExprKind::Lit(Lit::new(LitKind::Num(v))),
                Span::new(0, 0),
                Type::Unit,
            )),
            _ => None,
        };

        if let Some(expr) = expr {
            self.advance();
            return Ok(expr);
        }

        if self.r#match(vec![TokenKind::OpenBracket]) {
            let expr = self.expression()?;
            self.consume(TokenKind::CloseBracket, "Expected a closing Bracket")?;
            return Ok(Expr::new(
                ExprKind::Grouping(Box::new(expr)),
                Span::new(0, 0),
                Type::Unit,
            ));
        }

        return Err(self.error(self.peek(), "Expected expression"));
    }

    fn consume(&mut self, token_kind: TokenKind, message: &str) -> Result<&Token, ParseError> {
        if self.check(token_kind) {
            return Ok(self.advance());
        };

        Err(self.error(self.peek(), message))
    }

    fn error(&self, token: &Token, message: &str) -> ParseError {
        crate::error(token, message);
        ParseError
    }
}
