#![feature(let_chains)]
use parser::{lexer::lexer, parse};

mod build_code;
// mod interpreter;
mod parser;

fn main() -> Result<(), ()> {
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
    let tokens = parse("-3+5*7");

    println!("{tokens:#?}");

    // println!("{}", build_from_tokens(&tokens));

    Ok(())
}
