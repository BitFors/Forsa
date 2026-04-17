//TODO
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Decode(#[from] DecodeError),
}

#[derive(Debug, Error)]
pub enum DecodeError {
    #[error("Temp")]
    Temp,
}
