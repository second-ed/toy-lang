use thiserror::Error;

#[derive(Debug, Error)]
pub enum ToyLangErr {
    #[error("tokeniser error")]
    Tokeniser,
    #[error("parser error")]
    Parser,
    #[error("runtime error")]
    Runtime,
}
