use crate::error::ToyLangErr;

mod error;
mod interpreter;
mod parser;
mod tokenisation;

pub fn interpret(source: &str) -> Result<(), ToyLangErr> {
    println!("{source}");
    let tokens = tokenisation::tokenise(source)?;
    let ast = parser::parse(tokens)?;
    interpreter::evaluate(ast)?;
    Ok(())
}
