#[derive(Debug, PartialEq, Eq)]
pub(crate) struct SpannedToken<'src> {
    pub token: Token<'src>,
    pub span: Span,
}

impl<'src> SpannedToken<'src> {
    pub(crate) fn new(token: Token<'src>, line: u64, col: u64) -> Self {
        Self {
            token,
            span: Span::new(line, col),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Span {
    pub(crate) line: u64,
    pub(crate) col: u64,
}

impl Span {
    pub(crate) fn new(line: u64, col: u64) -> Self {
        Self { line, col }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Token<'src> {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    DoubleQuote,
    SemiColon,
    Keyword(Keyword),
    Identifier(&'src str),
    Number(&'src str),
    StringLiteral(&'src str),
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Keyword {
    Fn,
    If,
    Else,
    Return,
}
