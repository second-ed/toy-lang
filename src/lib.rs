use crate::error::ToyLangErr;

mod error;
mod interpreter;
mod lexer;
mod parser;

pub fn interpret(source: &str) -> Result<(), ToyLangErr> {
    println!("{source}");
    let tokens = lexer::tokenise(source)?;
    let ast = parser::parse(tokens)?;
    interpreter::evaluate(ast)?;
    Ok(())
}
