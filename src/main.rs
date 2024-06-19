#![feature(let_chains)]
use error_handling::error;
use parser::parse;

mod build_code;
mod error_handling;
// mod interpreter;
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

    let code = "-3+rrddd*7"; // (-3) + (3 * 7)
    unsafe { CODE = code };

    let ast = match parse(code) {
        Some(ast) => ast,
        None => return,
    };
    println!("{ast:#?}");
}
