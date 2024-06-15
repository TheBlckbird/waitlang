use super::token::{Token, TokenKind};
use crate::parser::ast::{lit::TimeKind, Span};

pub fn parse_num(remaining: &mut String, code_index: &mut i32) -> Token {
    let start = *code_index;
    let mut num = String::new();
    let mut last_char = ' ';

    while let Some(next_char) = remaining.chars().next() {
        match next_char {
            '0'..='9' => num.push(next_char),
            _ => break,
        }

        last_char = remaining.remove(0);
        *code_index += 1;
    }

    remaining.insert(0, last_char);
    *code_index -= 1;

    let num = num.parse().unwrap();

    let time_kind = if remaining.starts_with("ms") {
        Some(TimeKind::Ms)
    } else if remaining.starts_with('s') {
        Some(TimeKind::Sec)
    } else if remaining.starts_with("min") {
        Some(TimeKind::Min)
    } else if remaining.starts_with('h') {
        Some(TimeKind::Hour)
    } else if remaining.starts_with('d') {
        Some(TimeKind::Day)
    } else if remaining.starts_with('w') {
        Some(TimeKind::Week)
    } else if remaining.starts_with('y') {
        Some(TimeKind::Year)
    } else {
        None
    };

    let token_kind = match time_kind {
        Some(time_kind) => TokenKind::Time(num, time_kind),
        None => TokenKind::Num(num),
    };

    Token::new(token_kind, Span::new(start, *code_index))
}
