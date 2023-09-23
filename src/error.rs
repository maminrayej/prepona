use crate::provide::{EdgeId, NodeId};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Node not found: {0:?}")]
    NodeNotFound(NodeId),
    #[error("Node already exists: {0:?}")]
    NodeExists(NodeId),
    #[error("Edge not found: {0:?} -> {1:?}: {2:?}")]
    EdgeNotFound(NodeId, NodeId, EdgeId),
    #[error("Edge already exists: {0:?} -> {1:?}: {2:?}")]
    EdgeExists(NodeId, NodeId, EdgeId),
    #[error("Input graph is not a DAG")]
    NotDAG,
}
