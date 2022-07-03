use std::collections::HashMap;
use std::ops::Index;

use itertools::Itertools;

use super::NodeId;

pub trait IdMap:
    Index<Self::VirtId, Output = Self::RealId> + Index<Self::RealId, Output = Self::VirtId>
{
    type VirtId;

    type RealId;
}

pub trait NodeIdMapProvider {
    type NodeIdMap: IdMap<VirtId = usize, RealId = NodeId>;

    fn id_map(&self) -> Self::NodeIdMap;
}

pub struct DefaultIdMap {
    real_to_virt: HashMap<NodeId, usize>,
    virt_to_real: Vec<NodeId>,
}

impl DefaultIdMap {
    pub fn new(nodes: impl Iterator<Item = NodeId>) -> Self {
        let virt_to_real = nodes.collect_vec();
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

impl IdMap for DefaultIdMap {
    type VirtId = usize;

    type RealId = NodeId;
}
