use std::collections::HashSet;

use crate::misc::UnionFind;
use crate::provide::{EdgeId, EdgeRef, Id, NodeId, Storage};
use crate::view::View;

pub struct Kruskal<'a, S>
where
    S: Storage,
{
    storage: &'a S,
    idmap: S::Map,
}

impl<'a, S> Kruskal<'a, S>
where
    S: EdgeRef,
{
    pub fn new(storage: &'a S) -> Self {
        Self {
            storage,
            idmap: storage.idmap(),
        }
    }

    pub fn exec<F>(self, weight_of: F) -> View<'a, S, HashSet<NodeId>, HashSet<EdgeId>>
    where
        F: Fn(&'a S::Node, &'a S::Node, &'a S::Edge) -> isize,
    {
        let mut union = UnionFind::new(self.storage.node_count());
        let mut node_set = HashSet::new();
        let mut edge_set = HashSet::new();

        let mut edges = self
            .storage
            .edges()
            .map(|(src, dst, edge)| (src, dst, edge.id(), weight_of(src, dst, edge)))
            .collect::<Vec<_>>();

        edges.sort_by_key(|(_src, _dst, _eid, weight)| *weight);

        for (src, dst, eid, _weight) in edges {
            let src_idx = self.idmap[src.id()];
            let dst_idx = self.idmap[dst.id()];

            if !union.same_set(src_idx, dst_idx) {
                node_set.insert(src.id());
                node_set.insert(dst.id());
                edge_set.insert(eid);

                union.union(src_idx, dst_idx);
            }
        }

        View::new(self.storage, node_set, edge_set)
    }
}
