use thiserror::Error;

#[derive(Debug, Error)]
pub enum ToyLangErr {
    #[error("tokenizer error")]
    Tokenizer,
    #[error("parser error")]
    Parser,
    #[error("runtime error")]
    Runtime,
}
