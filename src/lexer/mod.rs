use crate::{
    error::ToyLangErr,
    lexer::{
        keyword::Keyword,
        position::Position,
        token::{SpannedToken, Token},
    },
};
pub(crate) mod keyword;
pub(crate) mod position;
pub(crate) mod token;

pub(crate) fn tokenize(source: &str) -> Result<Vec<SpannedToken<'_>>, ToyLangErr> {
    let mut tokens: Vec<SpannedToken> = vec![];
    let mut line: u64 = 1;
    let mut col: u64 = 1;

    let mut chars = source.char_indices().peekable();

    while let Some((start_idx, current)) = chars.next() {
        if [' ', '\t'].contains(&current) {
            col += 1;
            continue;
        }
        if ['\n'].contains(&current) {
            line += 1;
            col = 1;
            continue;
        }

        let start_pos = Position::new(line, col);

        let token = match current {
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            '{' => Token::LeftBrace,
            '}' => Token::RightBrace,
            ';' => Token::SemiColon,
            '=' => Token::EqualSign,
            '>' => Token::GreaterThan,
            '<' => Token::LessThan,
            '"' => {
                let content_start = start_idx + current.len_utf8();
                let mut content_end = content_start;
                let mut closed = false;

                col += 1;

                while let Some(&(next_index, next_char)) = chars.peek() {
                    chars.next();

                    if next_char == '"' {
                        closed = true;
                        col += 1;
                        break;
                    }

                    content_end = next_index + next_char.len_utf8();
                    col += 1;
                }

                if !closed {
                    return Err(ToyLangErr::Tokenizer);
                }

                let text = &source[content_start..content_end];

                Token::StringLiteral(text)
            }
            c if (c.is_ascii_alphabetic() || c == '_') => {
                let mut end_idx = start_idx + current.len_utf16();
                while let Some(&(next_idx, next_char)) = chars.peek() {
                    if next_char.is_ascii_alphabetic()
                        || next_char == '_'
                        || next_char.is_ascii_digit()
                    {
                        chars.next();

                        end_idx = next_idx + next_char.len_utf16();
                        col += 1;
                    } else {
                        break;
                    }
                }
                let text = &source[start_idx..end_idx];
                match Keyword::from_str(text) {
                    Some(keyword) => Token::Keyword(keyword),
                    None => Token::Identifier(text),
                }
            }
            c if c.is_ascii_digit() => {
                let mut end_idx = start_idx + current.len_utf16();
                while let Some(&(next_idx, next_char)) = chars.peek() {
                    if next_char.is_ascii_digit() {
                        chars.next();

                        end_idx = next_idx + next_char.len_utf16();
                        col += 1;
                    } else {
                        break;
                    }
                }
                let text = &source[start_idx..end_idx];
                Token::Number(text)
            }
            _ => todo!("{} is not handled", current),
        };

        let end_pos = Position::new(line, col + 1);

        tokens.push(SpannedToken::new(token, start_pos, end_pos));

        col += 1;
    }
    dbg!(&tokens);
    Ok(tokens)
}
