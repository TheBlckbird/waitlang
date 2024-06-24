#![feature(let_chains)]
use std::process::exit;

use error_handling::error;
use interpreter::Interpreter;
use parser::{ast::stmt::StmtKind, parse};

mod build_code;
mod error_handling;
mod interpreter;
mod parser;

static mut CODE: &str = "";

fn main() {
    // let ast = parse(
    //     "
    //     user a = detect_user();

    //     wait(30s);

    //     if (a.bpm > 120) {
    //         wait(15s);
    //     }

    //     wait(a.impatience * 120);",
    // )?;

    // let mut interpreter = Interpreter::new();
    // interpreter.run(ast)?;

    //     let tokens = parse(
    //         "
    // a: user = detect_user();

    // wait(30s);

    // if (bpm > 120) {
    //     wait(15s);
    // }

    // wait(a * 120);
    // func wow(name: type) -> type {}
    // ",
    //     )
    //     .unwrap();

    let code = "(-3 + 3) * 7"; // (-3) + (3 * 7)
    unsafe { CODE = code };

    let ast = match parse(code) {
        Some(ast) => ast,
        None => return,
    };
    println!("{ast:#?}");

    let expr = match ast.program.first().unwrap().stmt_kind.clone() {
        StmtKind::Expr(expr) => expr.clone(),
        _ => {
            println!("Not an expression");
            exit(1);
        }
    };

    let interpreter = Interpreter::new();
    let res = interpreter.eval_expr(&expr);
    println!("{res:#?}");
}
