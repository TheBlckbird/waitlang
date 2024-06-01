use self::ast::Ast;

pub mod ast;
mod lexer;

pub fn parse(code: &str) -> Result<Ast, ()> {
    let mut ast = Ast::new();

    // while !remaining.is_empty() {
    //     match remaining.chars().next().unwrap() {
    //         _ => return Err(())
    //     }
    // }

    Ok(ast)
}
