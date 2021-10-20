use thiserror::Error;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Vertex with token: {0} is not found")]
    VertexNotFound(String),

    #[error("Edge with token: {0} is not found")]
    EdgeNotFound(String),

    #[error("Number of provided elements: {0} is not equal to {1}")]
    NotKElement(usize, usize),
}
