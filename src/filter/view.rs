use std::marker::PhantomData;

use crate::filter::Filter;
use crate::provide::{EdgeId, EdgeRef, Id, NodeId, NodeRef, Storage};

pub struct View<'a, S, NF, EF> {
    storage: &'a S,
    ncount: usize,
    nfilter: NF,
    efilter: EF,
}

impl<'a, S, NF, EF> View<'a, S, NF, EF> {
    pub fn new(storage: &'a S, nfilter: NF, efilter: EF) -> Self
    where
        S: NodeRef,
        NF: Filter<Item = NodeId>,
    {
        let ncount = storage.nodes().filter(|n| nfilter.filter(&n.id())).count();

        Self {
            storage,
            nfilter,
            efilter,
            ncount,
        }
    }
}

impl<'a, S, NF, EF> Storage for View<'a, S, NF, EF>
where
    S: Storage,
{
    type Node = S::Node;
    type Edge = S::Edge;
    type Dir = S::Dir;
    type Map = S::Map;

    fn idmap(&self) -> Self::Map {
        self.storage.idmap()
    }
}

pub struct FilteredNodes<'a, S, NF, I> {
    inner: I,
    nfilter: &'a NF,

    phantom_s: PhantomData<S>,
}

impl<'a, S, NF, I> Iterator for FilteredNodes<'a, S, NF, I>
where
    S: NodeRef + 'a,
    NF: Filter<Item = NodeId>,
    I: Iterator<Item = <S::Nodes<'a> as Iterator>::Item>,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.inner.next() {
            if self.nfilter.filter(&node.id()) {
                return Some(node);
            }
        }

        None
    }
}

impl<'a, S, NF, EF> NodeRef for View<'a, S, NF, EF>
where
    S: NodeRef,
    NF: Filter<Item = NodeId>,
{
    type Nodes<'b> = FilteredNodes<'b, S, NF, S::Nodes<'b>> where Self: 'b;
    type Succs<'b> = FilteredNodes<'b, S, NF, S::Succs<'b>> where Self: 'b;
    type Preds<'b> = FilteredNodes<'b, S, NF, S::Preds<'b>> where Self: 'b;

    fn has_node(&self, nid: NodeId) -> bool {
        self.storage.has_node(nid) && self.nfilter.filter(&nid)
    }

    fn node_count(&self) -> usize {
        self.ncount
    }

    fn node(&self, nid: NodeId) -> &Self::Node {
        if self.has_node(nid) {
            self.storage.node(nid)
        } else {
            panic!("Node not found: {0:?}", nid);
        }
    }

    fn nodes(&self) -> Self::Nodes<'_> {
        FilteredNodes {
            inner: self.storage.nodes(),
            nfilter: &self.nfilter,
            phantom_s: PhantomData,
        }
    }

    fn succs(&self, nid: NodeId) -> Self::Succs<'_> {
        FilteredNodes {
            inner: self.storage.succs(nid),
            nfilter: &self.nfilter,
            phantom_s: PhantomData,
        }
    }

    fn preds(&self, nid: NodeId) -> Self::Preds<'_> {
        FilteredNodes {
            inner: self.storage.preds(nid),
            nfilter: &self.nfilter,
            phantom_s: PhantomData,
        }
    }
}

pub struct FilteredAllEdges<'a, S, NF, EF, I> {
    inner: I,
    nfilter: &'a NF,
    efilter: &'a EF,

    phantom_s: PhantomData<S>,
}

impl<'a, S, NF, EF, I> Iterator for FilteredAllEdges<'a, S, NF, EF, I>
where
    S: EdgeRef + 'a,
    NF: Filter<Item = NodeId>,
    EF: Filter<Item = EdgeId>,
    I: Iterator<Item = <S::AllEdges<'a> as Iterator>::Item>,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((src, dst, edge)) = self.inner.next() {
            if self.nfilter.filter(&src.id())
                && self.nfilter.filter(&dst.id())
                && self.efilter.filter(&edge.id())
            {
                return Some((src, dst, edge));
            }
        }

        None
    }
}

pub struct FilteredEdges<'a, S, NF, EF, I> {
    inner: I,
    nfilter: &'a NF,
    efilter: &'a EF,

    phantom_s: PhantomData<S>,
}

impl<'a, S, NF, EF, I> Iterator for FilteredEdges<'a, S, NF, EF, I>
where
    S: EdgeRef + 'a,
    NF: Filter<Item = NodeId>,
    EF: Filter<Item = EdgeId>,
    I: Iterator<Item = <S::Incoming<'a> as Iterator>::Item>,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((dst, edge)) = self.inner.next() {
            if self.nfilter.filter(&dst.id()) && self.efilter.filter(&edge.id()) {
                return Some((dst, edge));
            }
        }

        None
    }
}

impl<'a, S, NF, EF> EdgeRef for View<'a, S, NF, EF>
where
    S: EdgeRef,
    NF: Filter<Item = NodeId>,
    EF: Filter<Item = EdgeId>,
{
    type AllEdges<'b> = FilteredAllEdges<'b, S, NF, EF, S::AllEdges<'b>> where Self: 'b;
    type Incoming<'b> = FilteredEdges<'b, S, NF, EF, S::Incoming<'b>> where Self: 'b;
    type Outgoing<'b> = FilteredEdges<'b, S, NF, EF, S::Outgoing<'b>> where Self: 'b;

    fn has_edge(&self, src: NodeId, dst: NodeId, eid: EdgeId) -> bool {
        self.storage.has_edge(src, dst, eid)
            && self.nfilter.filter(&src)
            && self.nfilter.filter(&dst)
            && self.efilter.filter(&eid)
    }

    fn edges(&self) -> Self::AllEdges<'_> {
        FilteredAllEdges {
            inner: self.storage.edges(),
            nfilter: &self.nfilter,
            efilter: &self.efilter,
            phantom_s: PhantomData,
        }
    }

    fn incoming(&self, nid: NodeId) -> Self::Incoming<'_> {
        FilteredEdges {
            inner: self.storage.incoming(nid),
            nfilter: &self.nfilter,
            efilter: &self.efilter,
            phantom_s: PhantomData,
        }
    }

    fn outgoing(&self, nid: NodeId) -> Self::Outgoing<'_> {
        FilteredEdges {
            inner: self.storage.outgoing(nid),
            nfilter: &self.nfilter,
            efilter: &self.efilter,
            phantom_s: PhantomData,
        }
    }
}
