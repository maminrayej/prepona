use std::collections::HashSet;

use crate::give::*;
use crate::misc::UnionFind;
use crate::view::{AllNodeSelector, EdgeSetSelector, View};

pub struct Kruskal<'a, S> {
    storage: &'a S,
}

impl<'a, S> Kruskal<'a, S>
where
    S: Edge,
{
    pub fn init(storage: &'a S) -> Self {
        Self { storage }
    }

    pub fn exec(
        &self,
        weight: impl Fn(NodeID, NodeID) -> isize,
    ) -> View<'a, S, AllNodeSelector<S>, EdgeSetSelector<S>> {
        let node_count = self.storage.node_count();

        let idmap = self.storage.idmap();

        let mut used_edges = HashSet::new();

        let mut sets = UnionFind::new(node_count);

        let mut edges: Vec<(NodeID, NodeID, isize)> = self
            .storage
            .edges()
            .map(|(src, dst)| (src, dst, weight(src, dst)))
            .collect();

        edges.sort_by_key(|(_, _, w)| *w);

        for (src, dst, _w) in edges {
            let src_vid = idmap[src];
            let dst_vid = idmap[dst];

            if sets.equiv(src_vid, dst_vid) {
                used_edges.insert((src, dst));

                sets.union(src_vid, dst_vid);
            }
        }

        View::new(
            self.storage,
            AllNodeSelector::new(),
            EdgeSetSelector::new(used_edges),
        )
    }
}
