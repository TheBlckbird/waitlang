use crate::parser::{
    ast::lit::TimeKind,
    lexer::token::{Token, TokenKind},
};

pub fn build_from_tokens(tokens: &Vec<Token>) -> String {
    let mut code = String::new();

    for token in tokens {
        let new = match &token.kind {
            TokenKind::Ident(ident) => ident.clone() + " ",
            TokenKind::Num(num) => num.to_string(),
            TokenKind::Time(num, time_kind) => format!(
                "{num}{}",
                match time_kind {
                    TimeKind::Ms => "ms",
                    TimeKind::Sec => "s",
                    TimeKind::Min => "min",
                    TimeKind::Hour => "h",
                    TimeKind::Day => "d",
                    TimeKind::Week => "w",
                    TimeKind::Year => "y",
                }
            ),
            TokenKind::Bool(bool) => bool.to_string(),
            _ => String::from(match token.kind {
                TokenKind::Add => "+",
                TokenKind::Sub => "-",
                TokenKind::Mul => "*",
                TokenKind::Div => "/",
                TokenKind::Mod => "%",
                TokenKind::And => "&&",
                TokenKind::Or => "||",
                TokenKind::Xor => "^^",
                TokenKind::EqEq => "==",
                TokenKind::Ne => "!=",
                TokenKind::Lt => "<",
                TokenKind::Le => "<=",
                TokenKind::Gt => ">",
                TokenKind::Ge => ">=",
                TokenKind::Not => "!",
                TokenKind::OpenBracket => "(",
                TokenKind::CloseBracket => ")",
                TokenKind::OpenCurlBracket => "{",
                TokenKind::CloseCurlBracket => "}",
                TokenKind::Col => ":",
                TokenKind::Semi => ";",
                TokenKind::Arrow => "->",
                TokenKind::Eq => "=",
                TokenKind::Func => "func ",
                TokenKind::Eof => "",
                _ => unreachable!(),
            }),
        };

        code.push_str(new.as_str());
    }

    code
}
