use std::ops::{Add, Range, Sub};

use anyhow::Result;

use super::{ProviderError, Storage};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct NodeId(pub(crate) usize);

impl NodeId {
    pub fn inner(&self) -> usize {
        self.0
    }
}

impl From<usize> for NodeId {
    fn from(val: usize) -> Self {
        NodeId(val)
    }
}

impl Add<usize> for NodeId {
    type Output = NodeId;

    fn add(self, rhs: usize) -> Self::Output {
        NodeId(self.0 + rhs)
    }
}

impl Sub<usize> for NodeId {
    type Output = NodeId;

    fn sub(self, rhs: usize) -> Self::Output {
        NodeId(self.0 - rhs)
    }
}

#[derive(Debug)]
pub struct NodeIdRange {
    iter: Range<usize>,
}

impl Iterator for NodeIdRange {
    type Item = NodeId;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|num| num.into())
    }
}

impl NodeId {
    pub fn until(&self, end: NodeId) -> NodeIdRange {
        NodeIdRange {
            iter: self.0..end.0,
        }
    }

    pub fn to(&self, end: NodeId) -> NodeIdRange {
        self.until(end + 1)
    }

    pub fn range(&self, len: usize) -> NodeIdRange {
        self.until(*self + len)
    }
}

pub trait NodeProvider: Storage {
    type Nodes<'a>: Iterator<Item = NodeId>
    where
        Self: 'a;
    type Successors<'a>: Iterator<Item = NodeId>
    where
        Self: 'a;
    type Predecessors<'a>: Iterator<Item = NodeId>
    where
        Self: 'a;

    fn contains_node(&self, node: NodeId) -> bool;

    fn node_count(&self) -> usize;

    fn nodes(&self) -> Self::Nodes<'_>;

    fn successors(&self, node: NodeId) -> Self::Successors<'_>;
    fn successors_checked(&self, node: NodeId) -> Result<Self::Successors<'_>> {
        if !self.contains_node(node) {
            return Err(ProviderError::InvalidNode(node).into());
        }

        Ok(self.successors(node))
    }

    fn predecessors(&self, node: NodeId) -> Self::Predecessors<'_>;
    fn predecessors_checked(&self, node: NodeId) -> Result<Self::Predecessors<'_>> {
        if !self.contains_node(node) {
            return Err(ProviderError::InvalidNode(node).into());
        }

        Ok(self.predecessors(node))
    }

    fn is_successor(&self, node: NodeId, successor: NodeId) -> bool;
    fn is_successor_checked(&self, node: NodeId, successor: NodeId) -> Result<bool> {
        if !self.contains_node(node) {
            return Err(ProviderError::InvalidNode(node).into());
        } else if !self.contains_node(successor) {
            return Err(ProviderError::InvalidNode(successor).into());
        }

        Ok(self.is_successor(node, successor))
    }

    fn is_predecessor(&self, node: NodeId, predecessor: NodeId) -> bool;
    fn is_predecessor_checked(&self, node: NodeId, predecessor: NodeId) -> Result<bool> {
        if !self.contains_node(node) {
            return Err(ProviderError::InvalidNode(node).into());
        } else if !self.contains_node(predecessor) {
            return Err(ProviderError::InvalidNode(predecessor).into());
        }

        Ok(self.is_predecessor(node, predecessor))
    }
}

pub trait AddNodeProvider: NodeProvider {
    fn add_node(&mut self, node: NodeId);

    #[allow(clippy::unit_arg)]
    fn add_node_checked(&mut self, node: NodeId) -> Result<()> {
        if self.contains_node(node) {
            return Err(ProviderError::DuplicatedNode(node).into());
        }

        Ok(self.add_node(node))
    }
}

pub trait DelNodeProvider: NodeProvider {
    fn del_node(&mut self, node: NodeId);

    #[allow(clippy::unit_arg)]
    fn del_node_checked(&mut self, node: NodeId) -> Result<()> {
        if !self.contains_node(node) {
            return Err(ProviderError::NodeDoesNotExist(node).into());
        }

        Ok(self.del_node(node))
    }
}
