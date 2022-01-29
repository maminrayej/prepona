use thiserror::Error;

#[derive(Error, Debug)]
pub enum ViewError {
    #[error("Inner does not contain vertex with id: {0}")]
    InnerVertexNotFound(usize),

    #[error("View does not contain vertex with id: {0}")]
    VertexNotFound(usize),

    #[error("View does not contain edge with id: {0}")]
    EdgeNotFound(usize),

    #[error("Inner does not contain edge with id: {0}")]
    InnerEdgeNotFound(usize),
}
