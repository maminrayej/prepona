use thiserror::Error;

#[derive(Debug, Error)]
pub enum AlgoError {
    #[error("{0}")]
    NotBipartite(String),

    #[error("Reason: {0}")]
    UndefinedConcept(String),

    #[error("Two sets are not disjoint")]
    NotDisjointSets
}
