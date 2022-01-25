use thiserror::Error;

#[derive(Debug, Error)]
pub enum AlgoError {
    #[error("Graph is not bipartite")]
    GraphIsNotBipartite
}