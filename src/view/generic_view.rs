use std::iter::Copied;
use std::marker::PhantomData;

use indexmap::IndexSet;
use itertools::Itertools;

use crate::provide::{Direction, EdgeProvider, NodeId, NodeProvider, Storage};

use super::FrozenView;

pub struct GenericView<'i, G> {
    inner: &'i G,
    nodes: IndexSet<NodeId>,
    edges: IndexSet<(NodeId, NodeId)>,
}

impl<'i, G> GenericView<'i, G> {
    pub fn new(inner: &'i G, nodes: IndexSet<NodeId>, edges: IndexSet<(NodeId, NodeId)>) -> Self {
        Self {
            inner,
            nodes,
            edges,
        }
    }
}

pub struct Successors<'a, Dir: Direction> {
    target_node: NodeId,
    inner: indexmap::set::Iter<'a, (NodeId, NodeId)>,
    phantom_dir: PhantomData<Dir>,
}

impl<'a, Dir: Direction> Successors<'a, Dir> {
    pub fn new(target_node: NodeId, inner: indexmap::set::Iter<'a, (NodeId, NodeId)>) -> Self {
        Self {
            target_node,
            inner,
            phantom_dir: PhantomData,
        }
    }
}

impl<'a, Dir: Direction> Iterator for Successors<'a, Dir> {
    type Item = NodeId;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.find_map(|(src_node, dst_node)| {
            if *src_node == self.target_node {
                Some(*dst_node)
            } else if Dir::is_undirected() && *dst_node == self.target_node {
                Some(*src_node)
            } else {
                None
            }
        })
    }
}

pub struct Predecessors<'a, Dir: Direction> {
    target_node: NodeId,
    inner: indexmap::set::Iter<'a, (NodeId, NodeId)>,
    phantom_dir: PhantomData<Dir>,
}

impl<'a, Dir: Direction> Predecessors<'a, Dir> {
    pub fn new(target_node: NodeId, inner: indexmap::set::Iter<'a, (NodeId, NodeId)>) -> Self {
        Self {
            target_node,
            inner,
            phantom_dir: PhantomData,
        }
    }
}

impl<'a, Dir: Direction> Iterator for Predecessors<'a, Dir> {
    type Item = NodeId;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.find_map(|(src_node, dst_node)| {
            if *dst_node == self.target_node {
                Some(*src_node)
            } else if Dir::is_undirected() && *src_node == self.target_node {
                Some(*dst_node)
            } else {
                None
            }
        })
    }
}
impl<'i, G> Storage for GenericView<'i, G>
where
    G: Storage,
{
    type Dir = G::Dir;
}

impl<'i, G> NodeProvider for GenericView<'i, G>
where
    G: NodeProvider,
{
    type Nodes<'a> = Copied<indexmap::set::Iter<'a, NodeId>> where Self: 'a;

    type Successors<'a> = Successors<'a, G::Dir> where Self: 'a;

    type Predecessors<'a> = Predecessors<'a, G::Dir> where Self: 'a;

    fn contains_node(&self, node: NodeId) -> bool {
        self.nodes.contains(&node)
    }

    fn node_count(&self) -> usize {
        self.nodes.len()
    }

    fn nodes(&self) -> Self::Nodes<'_> {
        self.nodes.iter().copied()
    }

    fn successors(&self, node: NodeId) -> Self::Successors<'_> {
        Successors::new(node, self.edges.iter())
    }

    fn predecessors(&self, node: NodeId) -> Self::Predecessors<'_> {
        Predecessors::new(node, self.edges.iter())
    }

    fn is_successor(&self, node: NodeId, successor: NodeId) -> bool {
        self.successors(node).contains(&successor)
    }

    fn is_predecessor(&self, node: NodeId, predecessor: NodeId) -> bool {
        self.predecessors(node).contains(&predecessor)
    }
}

impl<'i, G> EdgeProvider for GenericView<'i, G>
where
    G: EdgeProvider,
{
    type Edges<'a> = Copied<indexmap::set::Iter<'a, (NodeId, NodeId)>> where Self: 'a;

    type IncomingEdges<'a> = Self::Predecessors<'a> where Self: 'a;

    type OutgoingEdges<'a> = Self::Successors<'a> where Self: 'a;

    fn contains_edge(&self, src_node: NodeId, dst_node: NodeId) -> bool {
        self.edges.contains(&(src_node, dst_node))
            || (G::Dir::is_undirected() && self.edges.contains(&(dst_node, src_node)))
    }

    fn edge_count(&self) -> usize {
        self.edges.len()
    }

    fn edges(&self) -> Self::Edges<'_> {
        self.edges.iter().copied()
    }

    fn incoming_edges(&self, node: NodeId) -> Self::IncomingEdges<'_> {
        self.predecessors(node)
    }

    fn outgoing_edges(&self, node: NodeId) -> Self::OutgoingEdges<'_> {
        self.successors(node)
    }

    fn in_degree(&self, node: NodeId) -> usize {
        self.incoming_edges(node).count()
    }

    fn out_degree(&self, node: NodeId) -> usize {
        self.outgoing_edges(node).count()
    }
}

impl<'i, G> FrozenView for GenericView<'i, G>
where
    G: NodeProvider + EdgeProvider,
{
    type Graph = G;

    fn inner(&self) -> &Self::Graph {
        self.inner
    }
}
