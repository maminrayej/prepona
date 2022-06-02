use std::collections::HashSet;
use std::marker::PhantomData;

use crate::provide::{Direction, NodeId, Orientation};

pub struct Nodes<'a> {
    pub(crate) iter: indexmap::map::Keys<'a, NodeId, Vec<(NodeId, Orientation)>>,
}

impl<'a> Iterator for Nodes<'a> {
    type Item = NodeId;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().copied()
    }
}

pub struct Neighbors<'a> {
    pub(crate) iter: std::slice::Iter<'a, (NodeId, Orientation)>,
    pub(crate) seen: HashSet<NodeId>,
}

impl<'a> Iterator for Neighbors<'a> {
    type Item = NodeId;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.find_map(|(node, _)| {
            if !self.seen.contains(node) {
                self.seen.insert(*node);
                Some(*node)
            } else {
                None
            }
        })
    }
}

pub struct OrientedNeighbors<'a, Dir: Direction> {
    pub(crate) iter: std::slice::Iter<'a, (NodeId, Orientation)>,
    pub(crate) orientation: Orientation,
    pub(crate) phantom_dir: PhantomData<Dir>,
}

impl<'a, Dir: Direction> Iterator for OrientedNeighbors<'a, Dir> {
    type Item = NodeId;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.find_map(|(node, orientation)| {
            if self.orientation == *orientation || Dir::is_undirected() {
                Some(*node)
            } else {
                None
            }
        })
    }
}

pub struct Edges<'a> {
    pub(crate) iter: indexmap::set::Iter<'a, (NodeId, NodeId)>,
}

impl<'a> Iterator for Edges<'a> {
    type Item = (NodeId, NodeId);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().copied()
    }
}
