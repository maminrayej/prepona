use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

use crate::algo::spath::{DataEntry, PathView};
use crate::provide::{EdgeId, EdgeRef, Id, NodeId, Storage};

pub struct AStar<'a, S>
where
    S: Storage,
{
    storage: &'a S,
    idmap: S::Map,
    start: &'a S::Node,
    target: &'a S::Node,
}

impl<'a, S> AStar<'a, S>
where
    S: EdgeRef,
{
    pub fn new(storage: &'a S, start: &'a S::Node, target: &'a S::Node) -> Self {
        Self {
            storage,
            idmap: storage.idmap(),
            start,
            target,
        }
    }

    pub fn exec<F, H>(
        self,
        cost_fn: F,
        hue_of: H,
    ) -> PathView<'a, S, usize, HashSet<NodeId>, HashSet<EdgeId>>
    where
        F: Fn(&'a S::Node, &'a S::Node, &'a S::Edge) -> usize,
        H: Fn(&'a S::Node, &'a S::Node) -> usize,
    {
        let mut heap = BinaryHeap::new();
        let mut cost_of = vec![usize::MAX; self.storage.node_count()];
        let mut visited = vec![false; self.storage.node_count()];
        let mut node_set = HashSet::new();
        let mut used_edges = HashSet::new();

        let start_idx = self.idmap[self.start.id()];
        cost_of[start_idx] = 0;

        heap.push(DataEntry {
            data: self.start,
            comp: Reverse(hue_of(self.start, self.target)),
        });

        while let Some(DataEntry {
            data: src,
            comp: Reverse(_hue_cost),
        }) = heap.pop()
        {
            let src_idx = self.idmap[src.id()];

            if visited[src_idx] {
                continue;
            }

            for (dst, edge) in self.storage.outgoing(src.id()) {
                let dst_idx = self.idmap[dst.id()];

                if visited[dst_idx] {
                    continue;
                }

                let new_cost = cost_of[src_idx] + cost_fn(src, dst, edge);

                if new_cost < cost_of[dst_idx] {
                    used_edges.insert(DataEntry {
                        data: edge.id(),
                        comp: dst_idx,
                    });
                    cost_of[dst_idx] = new_cost;
                    heap.push(DataEntry {
                        data: dst,
                        comp: Reverse(new_cost + hue_of(dst, self.target)),
                    });
                }
            }

            node_set.insert(src.id());
            visited[src_idx] = true;
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

                if cost != usize::MAX {
                    Some((nid, cost))
                } else {
                    None
                }
            })
            .collect();

        PathView::new(self.storage, node_set, edge_set, dist)
    }
}
