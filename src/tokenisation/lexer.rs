use crate::{
    error::ToyLangErr,
    tokenisation::{
        keyword::Keyword,
        position::Position,
        token::{SpannedToken, Token},
    },
};
use std::{iter::Peekable, str::CharIndices};

pub(crate) struct Lexer<'src> {
    source: &'src str,
    chars: Peekable<CharIndices<'src>>,
    line: u64,
    col: u64,
}

impl<'src> Lexer<'src> {
    pub(crate) fn new(source: &'src str) -> Self {
        Self {
            source,
            chars: source.char_indices().peekable(),
            line: 1,
            col: 1,
        }
    }

    pub(crate) fn tokenise(mut self) -> Result<Vec<SpannedToken<'src>>, ToyLangErr> {
        let mut tokens: Vec<SpannedToken> = vec![];

        while let Some((start_idx, current)) = self.chars.next() {
            if [' ', '\t'].contains(&current) {
                self.col += 1;
                continue;
            }
            if ['\n'].contains(&current) {
                self.line += 1;
                self.col = 1;
                continue;
            }

            let start_pos = Position::new(self.line, self.col);

            let token = match current {
                // brackets
                '(' | ')' | '[' | ']' | '{' | '}' => Self::scan_bracket(current),
                // operations
                '+' | '-' | '/' | '*' => self.scan_operator(current),
                // punctuation
                ':' => Token::Colon,
                ';' => Token::SemiColon,
                ',' => Token::Comma,
                // comparison
                '!' | '=' | '>' | '<' => self.scan_comparison(current),
                '"' => self.scan_string(start_idx, current)?,
                c if (c.is_ascii_alphabetic() || c == '_') => self.scan_name(start_idx, current),
                c if c.is_ascii_digit() => self.scan_number(start_idx, current),
                _ => todo!("{} is not handled", current),
            };

            self.col += 1;
            let end_pos = Position::new(self.line, self.col);
            tokens.push(SpannedToken::new(token, start_pos, end_pos));
        }
        dbg!(&tokens);
        Ok(tokens)
    }

    fn consume_if_next(&mut self, expected: char) -> bool {
        if let Some(&(_, next)) = self.chars.peek()
            && next == expected
        {
            self.chars.next();
            self.col += 1;
            return true;
        }
        false
    }

    fn scan_bracket(current: char) -> Token<'src> {
        match current {
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            '[' => Token::LeftBracket,
            ']' => Token::RightBracket,
            '{' => Token::LeftBrace,
            '}' => Token::RightBrace,
            _ => unreachable!("Character is not a supported bracket type: {current}"),
        }
    }

    fn scan_operator(&mut self, current: char) -> Token<'src> {
        match current {
            '+' if self.consume_if_next('=') => Token::PlusEqual,
            '+' if self.consume_if_next('+') => Token::PlusPlus,
            '+' => Token::Plus,
            '-' if self.consume_if_next('>') => Token::ReturnAnnotation,
            '-' if self.consume_if_next('=') => Token::DashEqual,
            '-' => Token::Dash,
            '/' if self.consume_if_next('=') => Token::SlashEqual,
            '/' => Token::Slash,
            '*' if self.consume_if_next('=') => Token::StarEqual,
            '*' => Token::Star,
            _ => unreachable!("Character is not a supported operator type: {current}"),
        }
    }

    fn scan_comparison(&mut self, current: char) -> Token<'src> {
        match current {
            '=' if self.consume_if_next('=') => Token::EqualEqual,
            '!' if self.consume_if_next('=') => Token::NotEqual,
            '=' => Token::Equal,
            '>' if self.consume_if_next('=') => Token::GreaterEqual,
            '<' if self.consume_if_next('=') => Token::LessEqual,
            '>' => Token::GreaterThan,
            '<' => Token::LessThan,
            _ => unreachable!("Character is not a supported comparison type: {current}"),
        }
    }

    fn scan_number(&mut self, start_idx: usize, current: char) -> Token<'src> {
        let mut is_float = false;

        let mut end_idx = start_idx + current.len_utf16();
        while let Some(&(next_idx, next_char)) = self.chars.peek() {
            if next_char.is_ascii_digit() || next_char == '.' {
                self.chars.next();

                end_idx = next_idx + next_char.len_utf16();
                self.col += 1;
                if !is_float && next_char == '.' {
                    is_float = true;
                }
            } else {
                break;
            }
        }
        let text = &self.source[start_idx..end_idx];
        if is_float {
            Token::Float(text)
        } else {
            Token::Integer(text)
        }
    }

    fn scan_name(&mut self, start_idx: usize, current: char) -> Token<'src> {
        let mut end_idx = start_idx + current.len_utf16();
        while let Some(&(next_idx, next_char)) = self.chars.peek() {
            if next_char.is_ascii_alphabetic() || next_char == '_' || next_char.is_ascii_digit() {
                self.chars.next();

                end_idx = next_idx + next_char.len_utf16();
                self.col += 1;
            } else {
                break;
            }
        }
        let text = &self.source[start_idx..end_idx];
        match Keyword::from_str(text) {
            Some(keyword) => Token::Keyword(keyword),
            None => Token::Name(text),
        }
    }

    fn scan_string(&mut self, start_idx: usize, current: char) -> Result<Token<'src>, ToyLangErr> {
        let content_start = start_idx + current.len_utf8();
        let mut content_end = content_start;
        let mut closed = false;

        while let Some(&(next_index, next_char)) = self.chars.peek() {
            self.chars.next();

            if next_char == '"' {
                closed = true;
                self.col += 1;
                break;
            }

            content_end = next_index + next_char.len_utf8();
            self.col += 1;
        }

        if !closed {
            return Err(ToyLangErr::Tokeniser);
        }
        let text = &self.source[content_start..content_end];
        Ok(Token::StringLiteral(text))
    }
}

#[cfg(test)]
mod tests {
    use super::Lexer;
    use crate::tokenisation::{
        keyword::Keyword,
        position::Position,
        token::{SpannedToken, Token},
    };
    use test_case::test_case;

    fn case_1() -> Vec<(&'static str, Vec<SpannedToken<'static>>)> {
        let input = "i = 0;";
        let output = vec![
            SpannedToken::new(Token::Name("i"), Position::new(1, 1), Position::new(1, 2)),
            SpannedToken::new(Token::Equal, Position::new(1, 3), Position::new(1, 4)),
            SpannedToken::new(
                Token::Integer("0"),
                Position::new(1, 5),
                Position::new(1, 6),
            ),
            SpannedToken::new(Token::SemiColon, Position::new(1, 6), Position::new(1, 7)),
        ];
        vec![(input, output)]
    }

    #[allow(clippy::too_many_lines)]
    fn token_test_cases() -> Vec<(&'static str, Vec<SpannedToken<'static>>)> {
        vec![
            ("", vec![]),
            (
                "()",
                vec![
                    SpannedToken::new(Token::LeftParen, Position::new(1, 1), Position::new(1, 2)),
                    SpannedToken::new(Token::RightParen, Position::new(1, 2), Position::new(1, 3)),
                ],
            ),
            (
                "[]",
                vec![
                    SpannedToken::new(Token::LeftBracket, Position::new(1, 1), Position::new(1, 2)),
                    SpannedToken::new(
                        Token::RightBracket,
                        Position::new(1, 2),
                        Position::new(1, 3),
                    ),
                ],
            ),
            (
                "{}",
                vec![
                    SpannedToken::new(Token::LeftBrace, Position::new(1, 1), Position::new(1, 2)),
                    SpannedToken::new(Token::RightBrace, Position::new(1, 2), Position::new(1, 3)),
                ],
            ),
            (
                "+",
                vec![SpannedToken::new(
                    Token::Plus,
                    Position::new(1, 1),
                    Position::new(1, 2),
                )],
            ),
            (
                "+=",
                vec![SpannedToken::new(
                    Token::PlusEqual,
                    Position::new(1, 1),
                    Position::new(1, 3),
                )],
            ),
            (
                "++",
                vec![SpannedToken::new(
                    Token::PlusPlus,
                    Position::new(1, 1),
                    Position::new(1, 3),
                )],
            ),
            (
                "-",
                vec![SpannedToken::new(
                    Token::Dash,
                    Position::new(1, 1),
                    Position::new(1, 2),
                )],
            ),
            (
                "-=",
                vec![SpannedToken::new(
                    Token::DashEqual,
                    Position::new(1, 1),
                    Position::new(1, 3),
                )],
            ),
            (
                "->",
                vec![SpannedToken::new(
                    Token::ReturnAnnotation,
                    Position::new(1, 1),
                    Position::new(1, 3),
                )],
            ),
            (
                "/",
                vec![SpannedToken::new(
                    Token::Slash,
                    Position::new(1, 1),
                    Position::new(1, 2),
                )],
            ),
            (
                "/=",
                vec![SpannedToken::new(
                    Token::SlashEqual,
                    Position::new(1, 1),
                    Position::new(1, 3),
                )],
            ),
            (
                "*",
                vec![SpannedToken::new(
                    Token::Star,
                    Position::new(1, 1),
                    Position::new(1, 2),
                )],
            ),
            (
                "*=",
                vec![SpannedToken::new(
                    Token::StarEqual,
                    Position::new(1, 1),
                    Position::new(1, 3),
                )],
            ),
            (
                ":",
                vec![SpannedToken::new(
                    Token::Colon,
                    Position::new(1, 1),
                    Position::new(1, 2),
                )],
            ),
            (
                ";",
                vec![SpannedToken::new(
                    Token::SemiColon,
                    Position::new(1, 1),
                    Position::new(1, 2),
                )],
            ),
            (
                ",",
                vec![SpannedToken::new(
                    Token::Comma,
                    Position::new(1, 1),
                    Position::new(1, 2),
                )],
            ),
            (
                "=",
                vec![SpannedToken::new(
                    Token::Equal,
                    Position::new(1, 1),
                    Position::new(1, 2),
                )],
            ),
            (
                "==",
                vec![SpannedToken::new(
                    Token::EqualEqual,
                    Position::new(1, 1),
                    Position::new(1, 3),
                )],
            ),
            (
                "!=",
                vec![SpannedToken::new(
                    Token::NotEqual,
                    Position::new(1, 1),
                    Position::new(1, 3),
                )],
            ),
            (
                ">",
                vec![SpannedToken::new(
                    Token::GreaterThan,
                    Position::new(1, 1),
                    Position::new(1, 2),
                )],
            ),
            (
                "<",
                vec![SpannedToken::new(
                    Token::LessThan,
                    Position::new(1, 1),
                    Position::new(1, 2),
                )],
            ),
            (
                ">=",
                vec![SpannedToken::new(
                    Token::GreaterEqual,
                    Position::new(1, 1),
                    Position::new(1, 3),
                )],
            ),
            (
                "<=",
                vec![SpannedToken::new(
                    Token::LessEqual,
                    Position::new(1, 1),
                    Position::new(1, 3),
                )],
            ),
            (
                "fn",
                vec![SpannedToken::new(
                    Token::Keyword(Keyword::Fn),
                    Position::new(1, 1),
                    Position::new(1, 3),
                )],
            ),
            (
                "if",
                vec![SpannedToken::new(
                    Token::Keyword(Keyword::If),
                    Position::new(1, 1),
                    Position::new(1, 3),
                )],
            ),
            (
                "else",
                vec![SpannedToken::new(
                    Token::Keyword(Keyword::Else),
                    Position::new(1, 1),
                    Position::new(1, 5),
                )],
            ),
            (
                "return",
                vec![SpannedToken::new(
                    Token::Keyword(Keyword::Return),
                    Position::new(1, 1),
                    Position::new(1, 7),
                )],
            ),
            (
                "true",
                vec![SpannedToken::new(
                    Token::Keyword(Keyword::True),
                    Position::new(1, 1),
                    Position::new(1, 5),
                )],
            ),
            (
                "false",
                vec![SpannedToken::new(
                    Token::Keyword(Keyword::False),
                    Position::new(1, 1),
                    Position::new(1, 6),
                )],
            ),
            (
                "for",
                vec![SpannedToken::new(
                    Token::Keyword(Keyword::For),
                    Position::new(1, 1),
                    Position::new(1, 4),
                )],
            ),
            (
                "in",
                vec![SpannedToken::new(
                    Token::Keyword(Keyword::In),
                    Position::new(1, 1),
                    Position::new(1, 3),
                )],
            ),
            (
                "print(\"Hello World\")",
                vec![
                    SpannedToken::new(
                        Token::Name("print"),
                        Position::new(1, 1),
                        Position::new(1, 6),
                    ),
                    SpannedToken::new(Token::LeftParen, Position::new(1, 6), Position::new(1, 7)),
                    SpannedToken::new(
                        Token::StringLiteral("Hello World"),
                        Position::new(1, 7),
                        Position::new(1, 20),
                    ),
                    SpannedToken::new(
                        Token::RightParen,
                        Position::new(1, 20),
                        Position::new(1, 21),
                    ),
                ],
            ),
            (
                "0",
                vec![SpannedToken::new(
                    Token::Integer("0"),
                    Position::new(1, 1),
                    Position::new(1, 2),
                )],
            ),
            (
                "0.0",
                vec![SpannedToken::new(
                    Token::Float("0.0"),
                    Position::new(1, 1),
                    Position::new(1, 4),
                )],
            ),
        ]
    }

    #[test_case(token_test_cases())]
    #[test_case(case_1())]
    fn test_tokenise(inputs: Vec<(&'static str, Vec<SpannedToken<'static>>)>) {
        let mismatches: Vec<String> = inputs
            .into_iter()
            .enumerate()
            .filter_map(
                |(case_number, (source, expected))| match Lexer::new(source).tokenise() {
                    Ok(actual) if actual == expected => None,

                    Ok(actual) => Some(format!(
                        "Case {case_number} failed for {source:?}\n\
                     Expected:\n{expected:#?}\n\
                     Actual:\n{actual:#?}"
                    )),

                    Err(error) => Some(format!(
                        "Case {case_number} failed for {source:?}\n\
                     Lexer error: {error:?}"
                    )),
                },
            )
            .collect();

        assert_eq!(
            mismatches,
            Vec::<String>::new(),
            "Tokenisation mismatches occurred"
        );
    }
}
