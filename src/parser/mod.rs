use crate::{error::ToyLangErr, lexer::token::SpannedToken, parser::ast::Ast};
pub(crate) mod ast;

pub(crate) fn parse(_tokens: Vec<SpannedToken>) -> Result<Ast, ToyLangErr> {
    Ok(Ast {})
}
