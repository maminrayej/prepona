use thiserror::Error;

#[derive(Debug, Error)]
pub enum AlgoError {
    #[error("Graph contains a negative cycle")]
    NegativeCycleDetected,
}
