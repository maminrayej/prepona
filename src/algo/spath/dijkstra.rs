use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use std::hash::Hash;

use crate::provide::{EdgeId, EdgeRef, Id, NodeId, Storage};

use super::PathView;

#[derive(Debug, Clone, Copy)]
struct DataEntry<T, C> {
    data: T,
    comp: C,
}
impl<T, C: PartialEq> PartialEq for DataEntry<T, C> {
    fn eq(&self, other: &Self) -> bool {
        self.comp.eq(&other.comp)
    }
}
impl<T, C: Eq> Eq for DataEntry<T, C> {}
impl<T, C: PartialOrd> PartialOrd for DataEntry<T, C> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.comp.partial_cmp(&other.comp)
    }
}
impl<T, C: Ord> Ord for DataEntry<T, C> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.comp.cmp(&other.comp)
    }
}
impl<T, C: Hash> Hash for DataEntry<T, C> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.comp.hash(state)
    }
}

pub struct Dijkstra<'a, S>
where
    S: Storage,
{
    storage: &'a S,
    idmap: S::Map,
    start: &'a S::Node,
}

impl<'a, S> Dijkstra<'a, S>
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

    pub fn exec<F>(self, cost_fn: F) -> PathView<'a, S, usize, HashSet<NodeId>, HashSet<EdgeId>>
    where
        F: Fn(&'a S::Node, &'a S::Node, &'a S::Edge) -> usize,
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
            comp: Reverse(0),
        });

        while let Some(DataEntry {
            data: src,
            comp: Reverse(cost),
        }) = heap.pop()
        {
            let node_idx = self.idmap[src.id()];

            if visited[node_idx] {
                continue;
            }

            for (dst, edge) in self.storage.outgoing(src.id()) {
                let dst_idx = self.idmap[dst.id()];

                if visited[dst_idx] {
                    continue;
                }

                let new_cost = cost_of[node_idx] + cost_fn(src, dst, edge);

                if new_cost < cost {
                    used_edges.insert(DataEntry {
                        data: edge.id(),
                        comp: dst_idx,
                    });
                    cost_of[dst_idx] = new_cost;
                    heap.push(DataEntry {
                        data: dst,
                        comp: Reverse(new_cost),
                    });
                }
            }

            node_set.insert(src.id());
            visited[node_idx] = true;
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
