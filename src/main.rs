use interpreter::Interpreter;
use parser::parse;

mod interpreter;
mod parser;

fn main() -> Result<(), ()> {
    let ast = parse(
        "
        user a = detect_user();

        wait(30s);

        if (a.bpm > 120) {
            wait(15s);
        }

        wait(a.impatience * 120);",
    )?;

    let mut interpreter = Interpreter::new();
    interpreter.run(ast)?;

    Ok(())
}
