use thiserror::Error;

use crate::provide::NodeID;

#[derive(Debug, Copy, Clone, Error)]
pub enum Error {
    #[error("Node: {0:?} not found")]
    NodeNotFound(NodeID),

    #[error("Node: {0:?} already exists")]
    NodeExists(NodeID),
}
