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
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    SemiColon,
    EqualSign,
    GreaterThan,
    LessThan,
    Keyword(Keyword),
    Identifier(&'src str),
    Number(&'src str),
    StringLiteral(&'src str),
}
