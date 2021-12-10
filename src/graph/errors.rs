use thiserror::Error;

/// Different kinds of errors that can happen during operations carried out in the storage module.
#[derive(Error, Debug)]
pub enum GraphError {
    #[error("There is already an edge from {0} to {1}")]
    MultiEdge(usize, usize),

    #[error(
        "Adding edge from {0} to itself creates a loop. Loop is not allowed in a simple graph"
    )]
    Loop(usize),
}
