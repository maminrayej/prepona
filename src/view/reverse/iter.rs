use crate::give::*;

pub struct Edges<'a, S>
where
    S: Edge<Dir = Directed> + 'a,
{
    pub(super) in_iter: S::AllEdges<'a>,
}

impl<'a, S> Iterator for Edges<'a, S>
where
    S: Edge<Dir = Directed> + 'a,
{
    type Item = (NodeID, NodeID);

    fn next(&mut self) -> Option<Self::Item> {
        self.in_iter.next().map(|(src, dst)| (dst, src))
    }
}
