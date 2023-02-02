use std::collections::{BinaryHeap, HashSet};

use crate::give::*;
use crate::view::{AllNodeSelector, EdgeSetSelector, View};

struct Element(NodeID, NodeID, usize);

impl PartialEq for Element {
    fn eq(&self, other: &Self) -> bool {
        self.2 == other.2
    }
}

impl Eq for Element {}

impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.2.partial_cmp(&other.2)
    }
}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub struct Prim<'a, S> {
    storage: &'a S,
}

impl<'a, S> Prim<'a, S>
where
    S: Edge,
{
    pub fn init(storage: &'a S) -> Self {
        Self { storage }
    }

    pub fn exec(
        &self,
        weight: impl Fn(NodeID, NodeID) -> usize,
    ) -> View<'a, S, AllNodeSelector<S>, EdgeSetSelector<S>> {
        let node_count = self.storage.node_count();

        let idmap = self.storage.idmap();

        let mut visited = vec![false; node_count];

        let mut heap = BinaryHeap::new();

        let mut used_edges = HashSet::new();

        let mut src = self.storage.nodes().take(1).next().unwrap();

        loop {
            let src_vid = idmap[src];

            for dst in self.storage.outgoing(src) {
                let dst_vid = idmap[dst];

                if visited[dst_vid] {
                    continue;
                }

                heap.push(Element(src, dst, weight(src, dst)));
            }

            visited[src_vid] = true;

            let Some(Element(s, d, _)) = heap.pop() else {
                break;
            };

            used_edges.insert((s, d));

            src = d;
        }

        View::new(
            self.storage,
            AllNodeSelector::new(),
            EdgeSetSelector::new(used_edges),
        )
    }
}
