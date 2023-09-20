use std::ops::Index;

use crate::provide::NodeId;

pub trait IdMap: Index<usize, Output = NodeId> + Index<NodeId, Output = usize> {}
