use crate::common::UnionFind;
use crate::provide::*;

pub struct Kruskal<'a, S> {
    storage: &'a S,
}

impl<'a, S> Kruskal<'a, S>
where
    S: Node + Edge,
{
    pub fn init(storage: &'a S) -> Self {
        Self { storage }
    }

    pub fn exec(&self, weight: impl Fn(NodeID, NodeID) -> isize) {
        let node_count = self.storage.node_count();
        let idmap = self.storage.idmap();

        let mut used_edges = vec![];

        let mut sets = UnionFind::new(node_count);

        let mut edges: Vec<(NodeID, NodeID, isize)> = self
            .storage
            .edges()
            .map(|(src, dst)| (src, dst, weight(src, dst)))
            .collect();

        edges.sort_by_key(|(_, _, w)| *w);

        for (src, dst, w) in edges {
            let src_vid = idmap[src];
            let dst_vid = idmap[dst];

            if sets.equiv(src_vid, dst_vid) {
                used_edges.push((src, dst, w));

                sets.union(src_vid, dst_vid);
            }
        }
    }
}
