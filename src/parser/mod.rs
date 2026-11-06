use crate::{error::ToyLangErr, lexer::token::Token, parser::ast::Ast};
pub(crate) mod ast;

pub(crate) fn parse(_tokens: Vec<Token>) -> Result<Ast, ToyLangErr> {
    Ok(Ast {})
}
