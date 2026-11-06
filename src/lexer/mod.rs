use crate::{error::ToyLangErr, lexer::token::Token};
pub(crate) mod token;

pub(crate) fn tokenize<'src>(_source: &'src str) -> Result<Vec<Token<'src>>, ToyLangErr> {
    Ok(vec![])
}
