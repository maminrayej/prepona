use crate::give::*;

pub struct Succs<'a, S>
where
    S: Edge,
{
    pub(super) fix_src: NodeID,
    pub(super) in_iter: S::Nodes<'a>,
    pub(super) storage: &'a S,
}

impl<'a, S> Iterator for Succs<'a, S>
where
    S: Edge,
{
    type Item = NodeID;

    fn next(&mut self) -> Option<Self::Item> {
        self.in_iter
            .find(|s| !self.storage.is_succ_of(self.fix_src, *s))
    }
}

pub struct Preds<'a, S>
where
    S: Edge,
{
    pub(super) fix_dst: NodeID,
    pub(super) in_iter: S::Nodes<'a>,
    pub(super) storage: &'a S,
}

impl<'a, S> Iterator for Preds<'a, S>
where
    S: Edge,
{
    type Item = NodeID;

    fn next(&mut self) -> Option<Self::Item> {
        self.in_iter
            .find(|p| !self.storage.is_pred_of(self.fix_dst, *p))
    }
}

pub struct Edges<'a, S>
where
    S: Edge,
{
    pub(super) product: itertools::Product<S::Nodes<'a>, S::Nodes<'a>>,
    pub(super) storage: &'a S,
}

impl<'a, S> Iterator for Edges<'a, S>
where
    S: Edge,
    S::Nodes<'a>: Clone,
{
    type Item = (NodeID, NodeID);

    fn next(&mut self) -> Option<Self::Item> {
        self.product
            .find(|(src, dst)| !self.storage.has_edge(*src, *dst))
    }
}
