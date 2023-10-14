use std::collections::HashSet;

use crate::algo::spath::{DataEntry, PathView};
use crate::error::{Error, Result};
use crate::provide::{EdgeId, EdgeRef, Id, NodeId, Storage};

pub struct BellmanFord<'a, S>
where
    S: Storage,
{
    storage: &'a S,
    idmap: S::Map,
    start: &'a S::Node,
}

impl<'a, S> BellmanFord<'a, S>
where
    S: EdgeRef,
{
    pub fn new(storage: &'a S, start: &'a S::Node) -> Self {
        Self {
            storage,
            idmap: storage.idmap(),
            start,
        }
    }

    pub fn exec<F>(
        self,
        cost_fn: F,
    ) -> Result<PathView<'a, S, isize, HashSet<NodeId>, HashSet<EdgeId>>>
    where
        F: Fn(&'a S::Node, &'a S::Node, &'a S::Edge) -> isize,
    {
        let mut cost_of = vec![isize::MAX; self.storage.node_count()];
        let mut node_set = HashSet::new();
        let mut used_edges = HashSet::new();

        let start_idx = self.idmap[self.start.id()];
        cost_of[start_idx] = 0;

        for _ in 1..self.storage.node_count() {
            for (src, dst, edge) in self.storage.edges() {
                let src_idx = self.idmap[src.id()];
                let dst_idx = self.idmap[dst.id()];

                let new_cost = cost_of[src_idx].saturating_add(cost_fn(src, dst, edge));

                if new_cost < cost_of[dst_idx] {
                    node_set.insert(src.id());
                    node_set.insert(dst.id());

                    cost_of[dst_idx] = new_cost;
                    used_edges.insert(DataEntry {
                        data: edge.id(),
                        comp: dst_idx,
                    });
                }
            }
        }

        for (src, dst, edge) in self.storage.edges() {
            let src_idx = self.idmap[src.id()];
            let dst_idx = self.idmap[dst.id()];

            let new_cost = cost_of[src_idx].saturating_add(cost_fn(src, dst, edge));

            if new_cost < cost_of[dst_idx] {
                return Err(Error::NegativeCycle);
            }
        }

        let edge_set: HashSet<EdgeId> = used_edges
            .into_iter()
            .map(|DataEntry { data, .. }| data)
            .collect();

        let dist = cost_of
            .into_iter()
            .enumerate()
            .filter_map(|(idx, cost)| {
                let nid = self.idmap[idx];

                if cost != isize::MAX {
                    Some((nid, cost))
                } else {
                    None
                }
            })
            .collect();

        Ok(PathView::new(self.storage, node_set, edge_set, dist))
    }
}
