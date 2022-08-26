use std::collections::HashMap;
use std::ops::Index;

use itertools::Itertools;

use super::{NodeId, NodeProvider};

pub trait IdMap<VirtId = usize, RealId = NodeId>:
    Index<VirtId, Output = RealId> + Index<RealId, Output = VirtId>
{
    fn new(graph: &impl NodeProvider) -> Self;
}

pub struct DefaultIdMap {
    real_to_virt: HashMap<NodeId, usize>,
    virt_to_real: Vec<NodeId>,
}

impl IdMap for DefaultIdMap {
    fn new(graph: &impl NodeProvider) -> Self {
        let virt_to_real = graph.nodes().collect_vec();
        let real_to_virt = virt_to_real
            .iter()
            .copied()
            .enumerate()
            .map(|(index, node)| (node, index))
            .collect();

        Self {
            real_to_virt,
            virt_to_real,
        }
    }
}

impl Index<NodeId> for DefaultIdMap {
    type Output = usize;

    fn index(&self, index: NodeId) -> &Self::Output {
        &self.real_to_virt[&index]
    }
}

impl Index<usize> for DefaultIdMap {
    type Output = NodeId;

    fn index(&self, index: usize) -> &Self::Output {
        &self.virt_to_real[index]
    }
}
