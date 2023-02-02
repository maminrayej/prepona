use thiserror::Error;

use crate::give::NodeID;

#[derive(Debug, Copy, Clone, Error)]
pub enum Error {
    #[error("Node: {0:?} not found")]
    NodeNotFound(NodeID),

    #[error("Node: {0:?} already exists")]
    NodeExists(NodeID),

    #[error("Edge: {0:?} -> {1:?} not found")]
    EdgeNotFound(NodeID, NodeID),
}
