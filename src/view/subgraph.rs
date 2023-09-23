use std::marker::PhantomData;

use crate::provide::{NodeId, NodeRef, Storage};

use super::Filter;

pub struct Subgraph<'a, S, NF, EF> {
    storage: &'a S,
    ncount: usize,
    nfilter: NF,
    efilter: EF,
}

impl<'a, S, NF, EF> Subgraph<'a, S, NF, EF> {
    pub fn new(storage: &'a S, nfilter: NF, efilter: EF) -> Self {
        /* TODO: compute node_count once */
        let ncount = 0;

        Self {
            storage,
            nfilter,
            efilter,
            ncount,
        }
    }
}

impl<'a, S, NF, EF> Storage for Subgraph<'a, S, NF, EF>
where
    S: Storage,
{
    type Node = S::Node;
    type Edge = S::Edge;
    type Dir = S::Dir;
    type Map = S::Map;

    fn idmap(&self) -> Self::Map {
        todo!()
    }
}

pub struct FilteredNodes<'a, S, NF, I>
where
    S: Storage + 'a,
    I: Iterator<Item = (NodeId, &'a S::Node)>,
{
    inner: I,
    nfilter: &'a NF,

    phantom_s: PhantomData<S>,
}

impl<'a, S, NF, I> Iterator for FilteredNodes<'a, S, NF, I>
where
    S: NodeRef + 'a,
    NF: Filter<Item = NodeId>,
    I: Iterator<Item = (NodeId, &'a S::Node)>,
{
    type Item = (NodeId, &'a S::Node);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((nid, node)) = self.inner.next() {
            if self.nfilter.select(&nid) {
                return Some((nid, node));
            }
        }

        None
    }
}

impl<'a, S, NF, EF> NodeRef for Subgraph<'a, S, NF, EF>
where
    S: NodeRef,
    NF: Filter<Item = NodeId>,
{
    type Nodes<'b> = FilteredNodes<'b, S, NF, S::Nodes<'b>> where Self: 'b;
    type Succs<'b> = FilteredNodes<'b, S, NF, S::Succs<'b>> where Self: 'b;
    type Preds<'b> = FilteredNodes<'b, S, NF, S::Preds<'b>> where Self: 'b;

    fn contains_node(&self, nid: NodeId) -> bool {
        self.storage.contains_node(nid) && self.nfilter.select(&nid)
    }

    fn node_count(&self) -> usize {
        self.ncount
    }

    fn node(&self, nid: NodeId) -> &Self::Node {
        todo!()
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
