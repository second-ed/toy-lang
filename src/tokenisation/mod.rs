use crate::{
    error::ToyLangErr,
    tokenisation::{lexer::Lexer, token::SpannedToken},
};

pub(crate) mod keyword;
pub(crate) mod lexer;
pub(crate) mod position;
pub(crate) mod token;

pub(crate) fn tokenise(source: &str) -> Result<Vec<SpannedToken<'_>>, ToyLangErr> {
    Lexer::new(source).tokenise()
}
