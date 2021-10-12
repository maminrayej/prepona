use thiserror::Error;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Vertex with token: {0} is not found")]
    VertexNotFound(String),

    #[error("Edge with token: {0} is not found")]
    EdgeNotFound(String),
}
