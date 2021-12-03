use thiserror::Error;

/// Different kinds of errors that can happen during operations carried out in the storage module.
#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Provided vertex token is not valid: {0}")]
    InvalidVertexToken(usize),

    #[error("Provided edge token is not valid: {0}")]
    InvalidEdgeToken(usize),

    #[error("There are no more vertex tokens available")]
    NoMoreVertexToken,

    #[error("There are no more edge tokens available")]
    NoMoreEdgeToken,
}
