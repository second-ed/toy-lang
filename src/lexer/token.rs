#[derive(Debug, PartialEq, Eq)]
pub(crate) struct SpannedToken<'src> {
    pub token: Token<'src>,
    pub span: Span,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Span {
    line: u64,
    col: u64,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Token<'src> {
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
