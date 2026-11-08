use crate::{error::ToyLangErr, parsing::ast::Ast, tokenisation::token::SpannedToken};
pub(crate) mod ast;

pub(crate) fn parse(_tokens: Vec<SpannedToken>) -> Result<Ast, ToyLangErr> {
    Ok(Ast {})
}
