use std::collections::BinaryHeap;

use crate::provide::*;

const INF: usize = usize::MAX;

struct Element(NodeID, usize, usize);

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

pub struct AStar<'a, S> {
    storage: &'a S,
}

impl<'a, S> AStar<'a, S>
where
    S: Node + Edge,
{
    pub fn init(storage: &'a S) -> Self {
        AStar { storage }
    }

    pub fn exec(
        &mut self,
        start: NodeID,
        cost_of: impl Fn(NodeID, NodeID) -> usize,
        estimate_of: impl Fn(NodeID) -> usize,
    ) {
        let node_count = self.storage.node_count();
        let idmap = self.storage.idmap();

        let mut visited = vec![false; node_count];
        let mut costs = vec![INF; node_count];
        let mut used_edge = vec![None; node_count];
        let mut heap = BinaryHeap::new();

        let start_vid = idmap[start];

        costs[start_vid] = 0;
        heap.push(Element(start, 0, estimate_of(start)));

        while let Some(Element(node, cost, _)) = heap.pop() {
            let node_vid = idmap[node];

            if visited[node_vid] {
                continue;
            }

            for dst in self.storage.outgoing(node) {
                let dst_vid = idmap[dst];

                if visited[dst_vid] {
                    continue;
                }

                let new_cost = cost + cost_of(node, dst);
                let old_cost = costs[dst_vid];

                if new_cost < old_cost {
                    costs[dst_vid] = new_cost;
                    used_edge[dst_vid] = Some((node, dst));
                    heap.push(Element(dst, new_cost, new_cost + estimate_of(dst)));
                }
            }

            visited[node_vid] = true;
        }
    }
}
