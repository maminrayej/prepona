use itertools::Itertools;

use crate::provide::*;

pub struct Complement<'a, S> {
    storage: &'a S,
}

impl<'a, S> Complement<'a, S> {
    pub fn new(storage: &'a S) -> Self {
        Self { storage }
    }
}

impl<'a, S> Storage for Complement<'a, S>
where
    S: Storage,
{
    type Dir = S::Dir;

    // TODO: Fix this map
    type Map = S::Map;

    fn idmap(&self) -> Self::Map {
        todo!()
    }
}

pub struct ComplementSuccs<'a, S>
where
    S: Edge,
{
    src: NodeID,
    nodes: S::Nodes<'a>,
    storage: &'a S,
}

impl<'a, S> Iterator for ComplementSuccs<'a, S>
where
    S: Edge,
{
    type Item = NodeID;

    fn next(&mut self) -> Option<Self::Item> {
        self.nodes.find(|s| !self.storage.is_succ_of(self.src, *s))
    }
}

pub struct ComplementPreds<'a, S>
where
    S: Edge,
{
    dst: NodeID,
    nodes: S::Nodes<'a>,
    storage: &'a S,
}

impl<'a, S> Iterator for ComplementPreds<'a, S>
where
    S: Edge,
{
    type Item = NodeID;

    fn next(&mut self) -> Option<Self::Item> {
        self.nodes.find(|p| !self.storage.is_pred_of(self.dst, *p))
    }
}

impl<'b, S> Node for Complement<'b, S>
where
    S: Edge,
{
    #[rustfmt::skip]
    type Nodes<'a> = S::Nodes<'a> where Self: 'a;

    #[rustfmt::skip]
    type Succs<'a> = ComplementSuccs<'a ,S> where Self: 'a;

    #[rustfmt::skip]
    type Preds<'a> = ComplementPreds<'a, S> where Self: 'a;

    fn contains_node(&self, node: NodeID) -> bool {
        self.storage.contains_node(node)
    }

    fn nodes(&self) -> Self::Nodes<'_> {
        self.storage.nodes()
    }

    fn succs(&self, node: NodeID) -> Self::Succs<'_> {
        ComplementSuccs {
            src: node,
            nodes: self.nodes(),
            storage: self.storage,
        }
    }

    fn preds(&self, node: NodeID) -> Self::Preds<'_> {
        ComplementPreds {
            dst: node,
            nodes: self.nodes(),
            storage: self.storage,
        }
    }
}

pub struct ComplementEdges<'a, S>
where
    S: Edge,
{
    possible_edges: itertools::Product<S::Nodes<'a>, S::Nodes<'a>>,
    storage: &'a S,
}

impl<'a, S> Iterator for ComplementEdges<'a, S>
where
    S: Edge,
    S::Nodes<'a>: Clone,
{
    type Item = (NodeID, NodeID);

    fn next(&mut self) -> Option<Self::Item> {
        self.possible_edges
            .find(|(src, dst)| !self.storage.contains_edge(*src, *dst))
    }
}

impl<'b, S> Edge for Complement<'b, S>
where
    S: Edge,
    for<'c> S::Nodes<'c>: Clone,
{
    #[rustfmt::skip]
    type AllEdges<'a> = ComplementEdges<'a, S> where Self: 'a;

    #[rustfmt::skip]
    type Incoming<'a> = ComplementPreds<'a, S> where Self: 'a;

    #[rustfmt::skip]
    type Outgoing<'a> = ComplementSuccs<'a, S> where Self: 'a;

    fn contains_edge(&self, src: NodeID, dst: NodeID) -> bool {
        !self.storage.contains_edge(src, dst)
    }

    fn edges(&self) -> Self::AllEdges<'_> {
        ComplementEdges {
            possible_edges: self.storage.nodes().cartesian_product(self.storage.nodes()),
            storage: self.storage,
        }
    }

    fn incoming(&self, node: NodeID) -> Self::Incoming<'_> {
        ComplementPreds {
            dst: node,
            nodes: self.nodes(),
            storage: self.storage,
        }
    }

    fn outgoing(&self, node: NodeID) -> Self::Outgoing<'_> {
        ComplementSuccs {
            src: node,
            nodes: self.nodes(),
            storage: self.storage,
        }
    }
}
