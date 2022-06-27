use crate::provide::{Directed, EdgeProvider, NodeId, NodeProvider, Storage, Undirected};

use super::FrozenView;

pub struct DirectedView<'b, G> {
    inner: &'b G,
}

impl<'b, G> DirectedView<'b, G>
where
    G: EdgeProvider<Dir = Undirected>,
{
    pub fn init(graph: &'b G) -> Self {
        DirectedView { inner: graph }
    }
}

impl<'b, G> Storage for DirectedView<'b, G>
where
    G: Storage<Dir = Undirected>,
{
    type Dir = Directed;
}

impl<'b, G> NodeProvider for DirectedView<'b, G>
where
    G: NodeProvider<Dir = Undirected>,
{
    type Nodes<'a> = G::Nodes<'a> where Self: 'a;

    type Successors<'a> = G::Successors<'a> where Self: 'a;

    type Predecessors<'a> = G::Predecessors<'a> where Self: 'a;

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
        self.inner.successors(node)
    }

    fn predecessors(&self, node: NodeId) -> Self::Predecessors<'_> {
        self.inner.predecessors(node)
    }

    fn is_successor(&self, node: NodeId, successor: NodeId) -> bool {
        self.inner.is_successor(node, successor)
    }

    fn is_predecessor(&self, node: NodeId, predecessor: NodeId) -> bool {
        self.inner.is_predecessor(node, predecessor)
    }
}

pub struct DirectedEdges<'a, G>
where
    G: EdgeProvider<Dir = Undirected> + 'a,
{
    inner: G::Edges<'a>,
    next: Option<(NodeId, NodeId)>,
}

impl<'a, G> Iterator for DirectedEdges<'a, G>
where
    G: EdgeProvider<Dir = Undirected> + 'a,
{
    type Item = (NodeId, NodeId);

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map_or_else(
            || {
                self.next = self.inner.next();

                self.next
            },
            |(node1, node2)| Some((node2, node1)),
        )
    }
}

impl<'b, G> EdgeProvider for DirectedView<'b, G>
where
    G: EdgeProvider<Dir = Undirected>,
{
    type Edges<'a> = DirectedEdges<'a, G> where Self: 'a;

    type IncomingEdges<'a> = G::IncomingEdges<'a> where Self: 'a;

    type OutgoingEdges<'a> = G::OutgoingEdges<'a> where Self: 'a;

    fn contains_edge(&self, src_node: NodeId, dst_node: NodeId) -> bool {
        self.inner.contains_edge(src_node, dst_node)
    }

    fn edge_count(&self) -> usize {
        self.inner.edge_count()
    }

    fn edges(&self) -> Self::Edges<'_> {
        DirectedEdges {
            inner: self.inner.edges(),
            next: None,
        }
    }

    fn incoming_edges(&self, node: NodeId) -> Self::IncomingEdges<'_> {
        self.inner.incoming_edges(node)
    }

    fn outgoing_edges(&self, node: NodeId) -> Self::OutgoingEdges<'_> {
        self.inner.outgoing_edges(node)
    }

    fn in_degree(&self, node: NodeId) -> usize {
        self.inner.in_degree(node)
    }

    fn out_degree(&self, node: NodeId) -> usize {
        self.inner.out_degree(node)
    }
}

impl<'b, G> FrozenView for DirectedView<'b, G>
where
    G: EdgeProvider<Dir = Undirected>,
{
    type Graph = G;

    fn inner(&self) -> &Self::Graph {
        self.inner
    }
}
