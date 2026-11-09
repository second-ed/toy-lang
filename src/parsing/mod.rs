use crate::{
    error::ToyLangErr, parsing::ast::Ast, parsing::parser::Parser,
    tokenisation::token::SpannedToken,
};
pub(crate) mod ast;
pub(crate) mod parser;

pub(crate) fn parse<'src>(tokens: Vec<SpannedToken<'src>>) -> Result<Ast, ToyLangErr> {
    Parser::new(tokens).parse()
}
