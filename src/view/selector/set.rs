use std::collections::HashSet;
use std::marker::PhantomData;

use crate::give::NodeID;

use super::Selector;

// TODO: Should it contain &'a storage to prevent node_set to become invalid?
pub struct NodeSetSelector<S> {
    set: HashSet<NodeID>,

    phantom_s: PhantomData<S>,
}

impl<S> NodeSetSelector<S> {
    pub fn new(set: HashSet<NodeID>) -> Self {
        Self {
            set,
            phantom_s: PhantomData,
        }
    }
}

impl<S> Selector for NodeSetSelector<S> {
    type Storage = S;

    type Element = NodeID;

    fn select(&self, _storage: &Self::Storage, element: &Self::Element) -> bool {
        self.set.contains(element)
    }
}

pub struct EdgeSetSelector<S> {
    set: HashSet<(NodeID, NodeID)>,

    phantom_s: PhantomData<S>,
}

impl<S> EdgeSetSelector<S> {
    pub fn new(set: HashSet<(NodeID, NodeID)>) -> Self {
        Self {
            set,
            phantom_s: PhantomData,
        }
    }
}

impl<S> Selector for EdgeSetSelector<S> {
    type Storage = S;

    type Element = (NodeID, NodeID);

    fn select(&self, _storage: &Self::Storage, element: &Self::Element) -> bool {
        self.set.contains(element)
    }
}
