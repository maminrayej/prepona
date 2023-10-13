use std::marker::PhantomData;

use crate::filter::{Filter, View};
use crate::provide::{Directed, EdgeId, EdgeRef, NodeId, NodeRef, Storage};

use super::{FilteredAllEdges, FilteredEdges, FilteredNodes};

pub struct Reverse<'a, S, NF, EF> {
    view: View<'a, S, NF, EF>,
}

impl<'a, S, NF, EF> Storage for Reverse<'a, S, NF, EF>
where
    S: Storage<Dir = Directed>,
{
    type Node = S::Node;
    type Edge = S::Edge;
    type Dir = Directed;
    type Map = S::Map;

    fn idmap(&self) -> Self::Map {
        self.view.idmap()
    }
}

impl<'a, S, NF, EF> NodeRef for Reverse<'a, S, NF, EF>
where
    S: NodeRef<Dir = Directed>,
    NF: Filter<Item = NodeId>,
{
    type Nodes<'b> = FilteredNodes<'b, S, NF, S::Nodes<'b>> where Self: 'b;
    type Succs<'b> = FilteredNodes<'b, S, NF, S::Preds<'b>> where Self: 'b;
    type Preds<'b> = FilteredNodes<'b, S, NF, S::Succs<'b>> where Self: 'b;

    fn has_node(&self, nid: NodeId) -> bool {
        self.view.has_node(nid)
    }

    fn node_count(&self) -> usize {
        self.view.node_count()
    }

    fn node(&self, nid: NodeId) -> &Self::Node {
        self.view.node(nid)
    }

    fn nodes(&self) -> Self::Nodes<'_> {
        self.view.nodes()
    }

    fn succs(&self, nid: NodeId) -> Self::Succs<'_> {
        self.view.preds(nid)
    }

    fn preds(&self, nid: NodeId) -> Self::Preds<'_> {
        self.view.succs(nid)
    }
}

pub struct ReversedAllEdges<S, I> {
    inner: I,
    phantom_s: PhantomData<S>,
}

impl<'a, S, I> Iterator for ReversedAllEdges<S, I>
where
    S: EdgeRef + 'a,
    I: Iterator<Item = <S::AllEdges<'a> as Iterator>::Item>,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(src, dst, edge)| (dst, src, edge))
    }
}

impl<'a, S, NF, EF> EdgeRef for Reverse<'a, S, NF, EF>
where
    S: EdgeRef<Dir = Directed>,
    NF: Filter<Item = NodeId>,
    EF: Filter<Item = EdgeId>,
{
    type AllEdges<'b> = ReversedAllEdges<S, FilteredAllEdges<'b, S, NF, EF, S::AllEdges<'b>>> where Self: 'b;
    type Incoming<'b> = FilteredEdges<'b, S, NF, EF, S::Outgoing<'b>> where Self: 'b;
    type Outgoing<'b> = FilteredEdges<'b, S, NF, EF, S::Incoming<'b>> where Self: 'b;

    fn has_edge(&self, src: NodeId, dst: NodeId, eid: EdgeId) -> bool {
        self.view.has_edge(dst, src, eid)
    }

    fn edges(&self) -> Self::AllEdges<'_> {
        ReversedAllEdges {
            inner: self.view.edges(),
            phantom_s: PhantomData,
        }
    }

    fn incoming(&self, nid: NodeId) -> Self::Incoming<'_> {
        self.view.outgoing(nid)
    }

    fn outgoing(&self, nid: NodeId) -> Self::Outgoing<'_> {
        self.view.incoming(nid)
    }
}
