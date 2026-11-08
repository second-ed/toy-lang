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
