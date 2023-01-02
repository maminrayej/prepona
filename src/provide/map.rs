use std::ops;

use crate::provide::NodeID;

pub trait IDMap: ops::Index<usize, Output = NodeID> + ops::Index<NodeID, Output = usize> {}
