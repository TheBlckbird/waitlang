use crate::{
    parser::{ast::Span, lexer::token::Token},
    CODE,
};
use colored::Colorize;
use std::process::exit;

enum ReportKind {
    Error,
    Warning,
}

pub fn warn(token: &Token, message: &str) {
    fancy_report(token.span, message, ReportKind::Warning)
}

pub fn error(token: &Token, message: &str) {
    fancy_report(token.span, message, ReportKind::Error);
}

fn fancy_report(span: Span, message: &str, kind: ReportKind) {
    eprint!(
        "{}: ",
        match kind {
            ReportKind::Error => "error".red(),
            ReportKind::Warning => "warning".yellow(),
        }
        .bold()
    );
    eprintln!("{}", message.bold());

    let (line, char_in_line) = match get_line_by_char(span.end, unsafe { CODE }) {
        Some(line) => line,
        None => {
            eprintln!("Please tell me what you did. Open an issue on the GitHub Repo (https://github.com/TheBlckbird/waitlang) or reach out to me in some other way.");
            exit(1);
        }
    };

    let mut error = String::from(unsafe { CODE }.lines().nth(line - 1).unwrap());

    error.insert_str(
        0,
        format!(" {line} |     ")
            .as_str()
            .bold()
            .blue()
            .to_string()
            .as_str(),
    );
    error.push('\n');
    error.insert_str(0, format!("   {}     \n", "|".bold().blue()).as_str());
    error.push_str(format!("   {}     ", "|".bold().blue()).as_str());
    error.reserve(char_in_line);

    for _ in 0..char_in_line {
        error.push(' ');
    }

    // TODO: color and separate buffer
    let mut error_pointer = String::new();
    for _ in 0..span.end - span.start + 1 {
        error_pointer.push('^');
    }

    match kind {
        ReportKind::Error => {
            error.push_str(error_pointer.as_str().bold().red().to_string().as_str())
        }
        ReportKind::Warning => {
            error.push_str(error_pointer.as_str().bold().yellow().to_string().as_str())
        }
    };

    eprintln!("{error}");
}

/// Option<(line, char_in_line)>
///
/// You have to pass in span.end
fn get_line_by_char(char: usize, code: &str) -> Option<(usize, usize)> {
    let lines = code.lines();
    let mut total_length = 0;

    for (i, line) in lines.enumerate() {
        total_length += line.len();
        if total_length >= i {
            return Some((i + 1, total_length - char));
        }
    }

    None
}
