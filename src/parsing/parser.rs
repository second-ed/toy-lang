use crate::tokenisation::token::SpannedToken;

#[derive(Debug, PartialEq)]
struct Parser<'src> {
    tokens: Vec<SpannedToken<'src>>,
    idx: usize,
}

impl<'src> Parser<'src> {
    pub(crate) fn new(tokens: Vec<SpannedToken<'src>>) -> Self {
        Self { tokens, idx: 0 }
    }

    pub(crate) fn peek(&self) -> Option<&SpannedToken<'src>> {
        self.tokens.get(self.idx)
    }

    pub(crate) fn next(&mut self) -> Option<SpannedToken<'src>> {
        let token = self.tokens.get(self.idx)?.clone();
        self.idx += 1;
        Some(token)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        parsing::parser::Parser,
        tokenisation::{
            position::Position,
            token::{SpannedToken, Token},
        },
    };
    use test_case::test_case;

    #[test]
    fn initialise_parser() {
        assert_eq!(Parser::new(vec![]), Parser::new(vec![]));
    }

    #[test_case(
        vec![
            SpannedToken::new(Token::LeftParen, Position::new(1, 1), Position::new(1, 2)),
            SpannedToken::new(Token::RightParen, Position::new(1, 2), Position::new(1, 3)),
        ],
        Some(&SpannedToken::new(Token::LeftParen, Position::new(1, 1), Position::new(1, 2)))
    )]
    fn peek_parser(tokens: Vec<SpannedToken>, expected_result: Option<&SpannedToken>) {
        let parser = Parser::new(tokens);
        assert_eq!(parser.peek(), expected_result);
        assert_eq!(parser.idx, 0);
    }

    #[test_case(
        vec![
            SpannedToken::new(Token::LeftParen, Position::new(1, 1), Position::new(1, 2)),
            SpannedToken::new(Token::RightParen, Position::new(1, 2), Position::new(1, 3)),
        ],
        Some(&SpannedToken::new(Token::LeftParen, Position::new(1, 1), Position::new(1, 2)))
    )]
    fn next_parser(tokens: Vec<SpannedToken>, expected_result: Option<&SpannedToken>) {
        let mut parser = Parser::new(tokens);
        assert_eq!(parser.next().as_ref(), expected_result);
        assert_eq!(parser.idx, 1);
    }
}
