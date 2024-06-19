use self::{
    ident::parse_ident,
    num::parse_num,
    token::{Token, TokenKind},
};
use super::ast::Span;

mod ident;
mod num;
pub mod token;

pub fn lexer(code: &str) -> Result<Vec<Token>, ()> {
    let mut tokens = vec![];
    let mut remaining = String::from(code);
    let mut code_index = 0;

    while !remaining.is_empty() {
        match remaining.chars().next().unwrap() {
            '(' => tokens.push(make_simple_token(TokenKind::OpenBracket, code_index)),
            ')' => tokens.push(make_simple_token(TokenKind::CloseBracket, code_index)),
            '{' => tokens.push(make_simple_token(TokenKind::OpenCurlBracket, code_index)),
            '}' => tokens.push(make_simple_token(TokenKind::CloseCurlBracket, code_index)),
            '+' => tokens.push(make_simple_token(TokenKind::Add, code_index)),
            '*' => tokens.push(make_simple_token(TokenKind::Mul, code_index)),
            '/' => tokens.push(make_simple_token(TokenKind::Div, code_index)),
            '%' => tokens.push(make_simple_token(TokenKind::Mod, code_index)),
            '-' => {
                remaining.chars().next();
                let is_minus = match remaining.chars().next() {
                    Some(next_char) => next_char != '>',
                    None => true,
                };

                if is_minus {
                    tokens.push(make_simple_token(TokenKind::Sub, code_index));
                } else {
                    tokens.push(Token::new(
                        TokenKind::Arrow,
                        Span::new(code_index, code_index + 1),
                    ));
                }
            }
            ':' => tokens.push(make_simple_token(TokenKind::Col, code_index)),

            ';' => tokens.push(make_simple_token(TokenKind::Semi, code_index)),
            'a'..='z' | 'A'..='Z' | '_' => {
                // remaining.remove(0);
                tokens.push(parse_ident(&mut remaining, &mut code_index));
            }
            '&' => {
                remaining.remove(0);
                code_index += 1;

                if let Some(next_char) = remaining.chars().next()
                    && next_char == '&'
                {
                    tokens.push(Token::new(
                        TokenKind::And,
                        Span::new(code_index - 1, code_index),
                    ))
                } else {
                    return Err(());
                }
            }
            '|' => {
                remaining.remove(0);
                code_index += 1;

                if let Some(next_char) = remaining.chars().next()
                    && next_char == '|'
                {
                    tokens.push(Token::new(
                        TokenKind::Or,
                        Span::new(code_index - 1, code_index),
                    ))
                } else {
                    return Err(());
                }
            }
            '=' => {
                let is_eq = match remaining.chars().nth(1) {
                    Some(next_char) => next_char == '=',
                    None => false,
                };

                if is_eq {
                    tokens.push(Token::new(
                        TokenKind::Eq,
                        Span::new(code_index, code_index + 1),
                    ));

                    code_index += 1;
                    remaining.remove(0);
                } else {
                    tokens.push(Token::new(TokenKind::Eq, Span::from(code_index)))
                }
            }
            '^' => {
                remaining.remove(0);
                code_index += 1;

                if let Some(next_char) = remaining.chars().next()
                    && next_char == '^'
                {
                    tokens.push(Token::new(
                        TokenKind::Xor,
                        Span::new(code_index - 1, code_index),
                    ));
                } else {
                    return Err(());
                }
            }
            '!' => {
                tokens.push(make_simple_token(TokenKind::Not, code_index));

                let is_ne = match remaining.chars().nth(1) {
                    Some(next_char) => next_char == '=',
                    None => false,
                };

                if is_ne {
                    tokens.push(Token::new(
                        TokenKind::Ne,
                        Span::new(code_index, code_index + 1),
                    ));

                    code_index += 1;
                    remaining.remove(0);
                } else {
                    tokens.push(Token::new(TokenKind::Not, Span::from(code_index)))
                }
            }
            '<' => {
                tokens.push(make_simple_token(TokenKind::Not, code_index));

                let is_le = match remaining.chars().nth(1) {
                    Some(next_char) => next_char == '=',
                    None => false,
                };

                if is_le {
                    tokens.push(Token::new(
                        TokenKind::Le,
                        Span::new(code_index, code_index + 1),
                    ));

                    code_index += 1;
                    remaining.remove(0);
                } else {
                    tokens.push(Token::new(TokenKind::Lt, Span::from(code_index)))
                }
            }
            '>' => {
                tokens.push(make_simple_token(TokenKind::Not, code_index));

                let is_le = match remaining.chars().nth(1) {
                    Some(next_char) => next_char == '=',
                    None => false,
                };

                if is_le {
                    tokens.push(Token::new(
                        TokenKind::Ge,
                        Span::new(code_index, code_index + 1),
                    ));

                    code_index += 1;
                    remaining.remove(0);
                } else {
                    tokens.push(Token::new(TokenKind::Gt, Span::from(code_index)))
                }
            }
            '0'..='9' => tokens.push(parse_num(&mut remaining, &mut code_index)),
            ' ' | '\n' | '\r' => {}
            _ => return Err(()),
        }
        code_index += 1;
        remaining.remove(0);
    }

    tokens.push(Token::new(TokenKind::Eof, Span::from(code_index)));

    Ok(tokens)
}

fn make_simple_token(token_kind: TokenKind, code_index: usize) -> Token {
    Token::new(token_kind, Span::from(code_index))
}
