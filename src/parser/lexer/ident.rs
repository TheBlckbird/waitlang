use super::token::{Token, TokenKind};
use crate::parser::ast::Span;

pub fn parse_ident(remaining: &mut String, code_index: &mut i32) -> Token {
    let start = *code_index;
    let mut ident = String::new();
    let mut last_char = ' ';

    while let Some(next_char) = remaining.chars().next() {
        match next_char {
            'a'..='z' | 'A'..='Z' | '_' => ident.push(next_char),
            _ => break,
        }

        last_char = remaining.remove(0);
        *code_index += 1;
    }

    remaining.insert(0, last_char);
    *code_index -= 1;

    let token_kind = match ident.as_str() {
        "true" => TokenKind::Bool(true),
        "false" => TokenKind::Bool(false),
        "func" => TokenKind::Func,
        _ => TokenKind::Ident(ident),
    };

    Token::new(token_kind, Span::new(start, *code_index))
}
