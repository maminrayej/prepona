use rudy::rudymap::RudyMap;
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

pub struct RudyIdMap {
    real_to_virt: RudyMap<NodeId, usize>,
    virt_to_real: Vec<NodeId>,
}

impl IdMap for RudyIdMap {
    fn new(graph: &impl NodeProvider) -> Self {
        let virt_to_real = graph.nodes().collect_vec();
        let mut real_to_virt = RudyMap::new();
        let success = virt_to_real
            .iter()
            .enumerate()
            .map(|(index, node)| real_to_virt.insert(*node, index))
            .all(|duplicated| duplicated.is_none());
        debug_assert!(success);

        Self {
            real_to_virt,
            virt_to_real,
        }
    }
}

impl Index<NodeId> for RudyIdMap {
    type Output = usize;

    fn index(&self, index: NodeId) -> &Self::Output {
        &self.real_to_virt.get(index).unwrap()
    }
}

impl Index<usize> for RudyIdMap {
    type Output = NodeId;

    fn index(&self, index: usize) -> &Self::Output {
        &self.virt_to_real[index]
    }
}

impl rudy::Key for NodeId {
    type Bytes = <usize as rudy::Key>::Bytes;

    fn into_bytes(self) -> Self::Bytes {
        self.0.into_bytes()
    }

    fn from_bytes(bytes: Self::Bytes) -> Self {
        Self::from(usize::from_bytes(bytes))
    }
}
