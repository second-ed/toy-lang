use crate::lexer::{
    keyword::Keyword,
    position::{Position, Span},
};

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct SpannedToken<'src> {
    pub token: Token<'src>,
    pub span: Span,
}

impl<'src> SpannedToken<'src> {
    pub(crate) fn new(token: Token<'src>, start_pos: Position, end_pos: Position) -> Self {
        Self {
            token,
            span: Span::new(start_pos, end_pos),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Token<'src> {
    // brackets
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    // operations
    Plus,
    Dash,
    Slash,
    Star,
    // punctuation
    Colon,
    SemiColon,
    Comma,
    // comparison
    Equal,
    EqualEqual,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterEqual,
    LessEqual,
    // types
    Keyword(Keyword),
    Name(&'src str),
    Integer(&'src str),
    Float(&'src str),
    StringLiteral(&'src str),
}
