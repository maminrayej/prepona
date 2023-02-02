use crate::give::*;
use crate::view::selector::Selector;

pub struct NodeSelector<'a, S, NS, I>
where
    S: Node + 'a,
{
    pub(crate) storage: &'a S,
    pub(crate) in_iter: I,
    pub(crate) nselect: &'a NS,
}

impl<'a, S, NS, I> Iterator for NodeSelector<'a, S, NS, I>
where
    S: Node,
    NS: Selector<Storage = S, Element = NodeID>,
    I: Iterator<Item = NodeID>,
{
    type Item = NodeID;

    fn next(&mut self) -> Option<Self::Item> {
        self.in_iter.find(|n| self.nselect.select(self.storage, n))
    }
}

pub struct AllEdges<'a, S, NS, ES, I>
where
    S: Edge + 'a,
{
    pub(crate) storage: &'a S,
    pub(crate) in_iter: I,
    pub(crate) eselect: &'a ES,
    pub(crate) nselect: &'a NS,
}

impl<'a, S, NS, ES, I> Iterator for AllEdges<'a, S, NS, ES, I>
where
    S: Edge + 'a,
    NS: Selector<Storage = S, Element = NodeID>,
    ES: Selector<Storage = S, Element = (NodeID, NodeID)>,
    I: Iterator<Item = (NodeID, NodeID)>,
{
    type Item = (NodeID, NodeID);

    fn next(&mut self) -> Option<Self::Item> {
        self.in_iter.find(|(src, dst)| {
            self.nselect.select(self.storage, src)
                && self.nselect.select(self.storage, dst)
                && self.eselect.select(self.storage, &(*src, *dst))
        })
    }
}

pub struct Incoming<'a, S, NS, ES, I>
where
    S: Edge + 'a,
{
    pub(crate) fix_dst: NodeID,
    pub(crate) in_iter: I,
    pub(crate) storage: &'a S,
    pub(crate) nselect: &'a NS,
    pub(crate) eselect: &'a ES,
}

impl<'a, S, NS, ES, I> Iterator for Incoming<'a, S, NS, ES, I>
where
    S: Edge + 'a,
    NS: Selector<Storage = S, Element = NodeID>,
    ES: Selector<Storage = S, Element = (NodeID, NodeID)>,
    I: Iterator<Item = NodeID>,
{
    type Item = NodeID;

    fn next(&mut self) -> Option<Self::Item> {
        self.in_iter.find(|src| {
            self.nselect.select(self.storage, src)
                && self.eselect.select(self.storage, &(*src, self.fix_dst))
        })
    }
}

pub struct Outgoing<'a, S, NS, ES, I>
where
    S: Edge + 'a,
{
    pub(crate) fix_src: NodeID,
    pub(crate) in_iter: I,
    pub(crate) storage: &'a S,
    pub(crate) nselect: &'a NS,
    pub(crate) eselect: &'a ES,
}

impl<'a, S, NS, ES, I> Iterator for Outgoing<'a, S, NS, ES, I>
where
    S: Edge + 'a,
    NS: Selector<Storage = S, Element = NodeID>,
    ES: Selector<Storage = S, Element = (NodeID, NodeID)>,
    I: Iterator<Item = NodeID>,
{
    type Item = NodeID;

    fn next(&mut self) -> Option<Self::Item> {
        self.in_iter.find(|dst| {
            self.nselect.select(self.storage, dst)
                && self.eselect.select(self.storage, &(self.fix_src, *dst))
        })
    }
}
