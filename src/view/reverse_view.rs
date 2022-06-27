use crate::provide::{Directed, EdgeProvider, NodeId, NodeProvider, Storage};

use super::FrozenView;

pub struct ReverseView<'a, G> {
    inner: &'a G,
}

impl<'a, G> ReverseView<'a, G>
where
    G: EdgeProvider<Dir = Directed>,
{
    pub fn init(inner: &'a G) -> Self {
        ReverseView { inner }
    }
}

impl<'a, G> Storage for ReverseView<'a, G>
where
    G: Storage<Dir = Directed>,
{
    type Dir = G::Dir;
}

impl<'b, G> NodeProvider for ReverseView<'b, G>
where
    G: NodeProvider<Dir = Directed>,
{
    type Nodes<'a> = G::Nodes<'a> where Self: 'a;

    type Successors<'a> = G::Predecessors<'a> where Self: 'a;

    type Predecessors<'a> = G::Successors<'a> where Self: 'a;

    fn contains_node(&self, node: NodeId) -> bool {
        self.inner.contains_node(node)
    }

    fn node_count(&self) -> usize {
        self.inner.node_count()
    }

    fn nodes(&self) -> Self::Nodes<'_> {
        self.inner.nodes()
    }

    fn successors(&self, node: NodeId) -> Self::Successors<'_> {
        self.inner.predecessors(node)
    }

    fn predecessors(&self, node: NodeId) -> Self::Predecessors<'_> {
        self.inner.successors(node)
    }

    fn is_successor(&self, node: NodeId, successor: NodeId) -> bool {
        self.inner.is_predecessor(node, successor)
    }

    fn is_predecessor(&self, node: NodeId, predecessor: NodeId) -> bool {
        self.inner.is_successor(node, predecessor)
    }
}

pub struct ReversedEdges<'a, G: EdgeProvider + 'a> {
    iter: G::Edges<'a>,
}

impl<'a, G> Iterator for ReversedEdges<'a, G>
where
    G: EdgeProvider + 'a,
{
    type Item = (NodeId, NodeId);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(node1, node2)| (node2, node1))
    }
}

impl<'b, G> EdgeProvider for ReverseView<'b, G>
where
    G: EdgeProvider<Dir = Directed>,
{
    type Edges<'a> = ReversedEdges<'a, G> where Self: 'a;

    type IncomingEdges<'a> = G::OutgoingEdges<'a> where Self: 'a;

    type OutgoingEdges<'a> = G::IncomingEdges<'a> where Self: 'a;

    fn contains_edge(&self, src_node: NodeId, dst_node: NodeId) -> bool {
        self.inner.contains_edge(src_node, dst_node)
    }

    fn edge_count(&self) -> usize {
        self.inner.edge_count()
    }

    fn edges(&self) -> Self::Edges<'_> {
        ReversedEdges {
            iter: self.inner.edges(),
        }
    }

    fn incoming_edges(&self, node: NodeId) -> Self::IncomingEdges<'_> {
        self.inner.outgoing_edges(node)
    }

    fn outgoing_edges(&self, node: NodeId) -> Self::OutgoingEdges<'_> {
        self.inner.incoming_edges(node)
    }

    fn in_degree(&self, node: NodeId) -> usize {
        self.inner.out_degree(node)
    }

    fn out_degree(&self, node: NodeId) -> usize {
        self.inner.in_degree(node)
    }
}

impl<'b, G> FrozenView for ReverseView<'b, G>
where
    G: EdgeProvider<Dir = Directed>,
{
    type Graph = G;

    fn inner(&self) -> &Self::Graph {
        self.inner
    }
}
