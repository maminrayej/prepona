use anyhow::Result;

use super::{NodeId, NodeProvider, ProviderError};

pub trait EdgeProvider: NodeProvider {
    type Edges<'a>: Iterator<Item = (NodeId, NodeId)>
    where
        Self: 'a;

    type IncomingEdges<'a>: Iterator<Item = NodeId>
    where
        Self: 'a;

    type OutgoingEdges<'a>: Iterator<Item = NodeId>
    where
        Self: 'a;

    fn contains_edge(&self, src_node: NodeId, dst_node: NodeId) -> bool;
    fn contains_edge_checked(&self, src_node: NodeId, dst_node: NodeId) -> Result<bool> {
        if !self.contains_node(src_node) {
            return Err(ProviderError::InvalidNode(src_node).into());
        } else if !self.contains_node(dst_node) {
            return Err(ProviderError::InvalidNode(dst_node).into());
        }

        Ok(self.contains_edge(src_node, dst_node))
    }

    fn edge_count(&self) -> usize;

    fn edges(&self) -> Self::Edges<'_>;

    fn incoming_edges(&self, node: NodeId) -> Self::IncomingEdges<'_>;
    fn incoming_edges_checked(&self, node: NodeId) -> Result<Self::IncomingEdges<'_>> {
        if !self.contains_node(node) {
            return Err(ProviderError::InvalidNode(node).into());
        }

        Ok(self.incoming_edges(node))
    }

    fn outgoing_edges(&self, node: NodeId) -> Self::OutgoingEdges<'_>;
    fn outgoing_edges_checked(&self, node: NodeId) -> Result<Self::OutgoingEdges<'_>> {
        if !self.contains_node(node) {
            return Err(ProviderError::InvalidNode(node).into());
        }

        Ok(self.outgoing_edges(node))
    }

    fn in_degree(&self, node: NodeId) -> usize;
    fn in_degree_checked(&self, node: NodeId) -> Result<usize> {
        if !self.contains_node(node) {
            return Err(ProviderError::InvalidNode(node).into());
        }

        Ok(self.in_degree(node))
    }

    fn out_degree(&self, node: NodeId) -> usize;
    fn out_degree_checked(&self, node: NodeId) -> Result<usize> {
        if !self.contains_node(node) {
            return Err(ProviderError::InvalidNode(node).into());
        }

        Ok(self.out_degree(node))
    }
}

pub trait AddEdgeProvider: EdgeProvider {
    fn add_edge(&mut self, src_node: NodeId, dst_node: NodeId);

    #[allow(clippy::unit_arg)]
    fn add_edge_checked(&mut self, src_node: NodeId, dst_node: NodeId) -> Result<()> {
        if self.contains_edge_checked(src_node, dst_node)? {
            return Err(ProviderError::MultiEdge(src_node, dst_node).into());
        }

        Ok(self.add_edge(src_node, dst_node))
    }
}

pub trait DelEdgeProvider: EdgeProvider {
    fn del_edge(&mut self, src_node: NodeId, dst_node: NodeId);

    #[allow(clippy::unit_arg)]
    fn del_edge_checked(&mut self, src_node: NodeId, dst_node: NodeId) -> Result<()> {
        if !self.contains_edge_checked(src_node, dst_node)? {
            return Err(ProviderError::EdgeDoesNotExist(src_node, dst_node).into());
        }

        Ok(self.del_edge(src_node, dst_node))
    }
}
