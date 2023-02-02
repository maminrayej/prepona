mod iter;
pub use iter::*;

use itertools::Itertools;

use crate::give::*;

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

impl<'b, S> Node for Complement<'b, S>
where
    S: Edge,
{
    #[rustfmt::skip]
    type Nodes<'a> = S::Nodes<'a> where Self: 'a;

    #[rustfmt::skip]
    type Succs<'a> = Succs<'a ,S> where Self: 'a;

    #[rustfmt::skip]
    type Preds<'a> = Preds<'a, S> where Self: 'a;

    fn has_node(&self, node: NodeID) -> bool {
        self.storage.has_node(node)
    }

    fn nodes(&self) -> Self::Nodes<'_> {
        self.storage.nodes()
    }

    fn succs(&self, node: NodeID) -> Self::Succs<'_> {
        Succs {
            fix_src: node,
            in_iter: self.nodes(),
            storage: self.storage,
        }
    }

    fn preds(&self, node: NodeID) -> Self::Preds<'_> {
        Preds {
            fix_dst: node,
            in_iter: self.nodes(),
            storage: self.storage,
        }
    }
}

impl<'b, S> Edge for Complement<'b, S>
where
    S: Edge,
    for<'c> S::Nodes<'c>: Clone,
{
    #[rustfmt::skip]
    type AllEdges<'a> = Edges<'a, S> where Self: 'a;

    #[rustfmt::skip]
    type Incoming<'a> = Preds<'a, S> where Self: 'a;

    #[rustfmt::skip]
    type Outgoing<'a> = Succs<'a, S> where Self: 'a;

    fn has_edge(&self, src: NodeID, dst: NodeID) -> bool {
        !self.storage.has_edge(src, dst)
    }

    fn edges(&self) -> Self::AllEdges<'_> {
        Edges {
            product: self.storage.nodes().cartesian_product(self.storage.nodes()),
            storage: self.storage,
        }
    }

    fn incoming(&self, node: NodeID) -> Self::Incoming<'_> {
        Preds {
            fix_dst: node,
            in_iter: self.nodes(),
            storage: self.storage,
        }
    }

    fn outgoing(&self, node: NodeID) -> Self::Outgoing<'_> {
        Succs {
            fix_src: node,
            in_iter: self.nodes(),
            storage: self.storage,
        }
    }
}
