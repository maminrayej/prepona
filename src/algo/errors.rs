use thiserror::Error;

#[derive(Debug, Error)]
pub enum AlgoError {
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),

    #[error("{0}")]
    NotBipartite(String),

    #[error("Reason: {0}")]
    UndefinedConcept(String),

    #[error("Two sets are not disjoint")]
    NotDisjointSets,
}
