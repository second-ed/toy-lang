use crate::{error::ToyLangErr, lexer::token::Token};
pub(crate) mod token;

pub(crate) fn tokenize(_source: &str) -> Result<Vec<Token>, ToyLangErr> {
    Ok(vec![])
}
