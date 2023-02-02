use std::ops;

use crate::give::NodeID;

pub trait IDMap: ops::Index<usize, Output = NodeID> + ops::Index<NodeID, Output = usize> {}
