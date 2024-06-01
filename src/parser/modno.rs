use self::ast::{
    block::Block,
    expr::{BinOp, Expr, ExprKind, Ident},
    lit::{Lit, LitKind, TimeKind},
    stmt::{Stmt, StmtKind},
    Ast, Span, Type,
};
use thin_vec::thin_vec;

pub mod ast;

pub fn parse(code: &str) -> Result<Ast, ()> {
    let mut ast = Ast::new();

    ast.program = vec![
        Stmt::new(
            StmtKind::VarBind {
                type_: Type::User,
                identifier: Ident::new("a", Span::from((1, 1))),
                value: Box::new(Expr::new(
                    ExprKind::FnCall(Ident::new("detect_user", Span::from((3, 3))), Box::new([])),
                    Span::from((2, 2)),
                    Type::User,
                )),
            },
            Span::from((0, 0)),
        ),
        Stmt::new(
            StmtKind::Expr(Box::new(Expr::new(
                ExprKind::FnCall(
                    Ident::new("wait", Span::from((6, 6))),
                    Box::new([Expr::new(
                        ExprKind::Lit(Lit::new(
                            LitKind::Time(30., TimeKind::Sec),
                            Span::from((7, 7)),
                            Type::Time,
                        )),
                        Span::from((6, 6)),
                        Type::Unit,
                    )]),
                ),
                Span::from((5, 5)),
                Type::Unit,
            ))),
            Span::from((4, 4)),
        ),
        Stmt::new(
            StmtKind::If(
                Box::new(Expr::new(
                    ExprKind::Binary(
                        BinOp::Gt,
                        Box::new(Expr::new(
                            ExprKind::FieldAcc(
                                Box::new(Expr::new(
                                    ExprKind::Ident(Ident::new("a", Span::from((13, 13)))),
                                    Span::from((12, 12)),
                                    Type::Unit, // TODO: Add struct type
                                )),
                                Ident::new("bpm", Span::from((14, 14))),
                            ),
                            Span::from((11, 11)),
                            Type::Number,
                        )),
                        Box::new(Expr::new(
                            ExprKind::Lit(Lit::new(
                                LitKind::Num(120.),
                                Span::from((16, 16)),
                                Type::Number,
                            )),
                            Span::from((15, 15)),
                            Type::Number,
                        )),
                    ),
                    Span::from((10, 10)),
                    Type::Bool,
                )),
                Box::new(Block {
                    stmts: Box::new([Stmt::new(
                        StmtKind::Expr(Box::new(Expr::new(
                            ExprKind::FnCall(
                                Ident::new("wait", Span::from((6, 6))),
                                Box::new([Expr::new(
                                    ExprKind::Lit(Lit::new(
                                        LitKind::Time(15., TimeKind::Sec),
                                        Span::from((7, 7)),
                                        Type::Time,
                                    )),
                                    Span::from((6, 6)),
                                    Type::Time,
                                )]),
                            ),
                            Span::from((5, 5)),
                            Type::Unit,
                        ))),
                        Span::from((4, 4)),
                    )]),
                    span: Span::from((17, 17)),
                    type_: Type::Unit,
                }),
                None,
            ),
            Span::from((8, 8)),
        ),
        Stmt::new(
            StmtKind::Expr(Box::new(Expr::new(
                ExprKind::FnCall(
                    Ident::new("wait", Span::from((20, 20))),
                    Box::new([Expr::new(
                        ExprKind::Binary(
                            BinOp::Mul,
                            Box::new(Expr::new(
                                ExprKind::FieldAcc(
                                    Box::new(Expr::new(
                                        ExprKind::Ident(Ident::new("a", Span::from((25, 25)))),
                                        Span::from((24, 24)),
                                        Type::Unit, // TODO: Add struct type
                                    )),
                                    Ident::new("impatience", Span::from((23, 23))),
                                ),
                                Span::from((22, 22)),
                                Type::Number,
                            )),
                            Box::new(Expr::new(
                                ExprKind::Lit(Lit::new(
                                    LitKind::Num(120.),
                                    Span::from((27, 27)),
                                    Type::Number,
                                )),
                                Span::from((26, 26)),
                                Type::Number,
                            )),
                        ),
                        Span::from((21, 21)),
                        Type::Unit,
                    )]),
                ),
                Span::from((19, 19)),
                Type::Unit,
            ))),
            Span::from((18, 18)),
        ),
    ];

    Ok(ast)
}
