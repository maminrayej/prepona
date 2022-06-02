use thiserror::Error;

use super::NodeId;

#[derive(Debug, Error)]
pub enum ProviderError {
    #[error("Provide node is invalid: {0:?}")]
    InvalidNode(NodeId),

    #[error("Provided node already exists in the provider: {0:?}")]
    DuplicatedNode(NodeId),

    #[error("Provided node does not exist in the provider: {0:?}")]
    NodeDoesNotExist(NodeId),

    #[error("Provided edge does not exist in the provider: ({0:?}, {0:?})")]
    EdgeDoesNotExist(NodeId, NodeId),

    #[error("Provided edge already exists in the provider: ({0:?}, {0:?})")]
    MultiEdge(NodeId, NodeId),
}
