use anyhow::Result;

use super::{ProviderError, Storage};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct NodeId(usize);

pub trait NodeProvider: Storage {
    type Nodes<'a>: Iterator<Item = NodeId>
    where
        Self: 'a;
    type Neighbors<'a>: Iterator<Item = NodeId>
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

    fn neighbors(&self, node: NodeId) -> Self::Neighbors<'_>;
    fn neighbors_checked(&self, node: NodeId) -> Result<Self::Neighbors<'_>> {
        if !self.contains_node(node) {
            return Err(ProviderError::InvalidNode(node).into());
        }

        Ok(self.neighbors(node))
    }

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
        if self.contains_node(node) {
            return Err(ProviderError::NodeDoesNotExist(node).into());
        }

        Ok(self.del_node(node))
    }
}
