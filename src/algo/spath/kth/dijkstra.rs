use std::collections::BinaryHeap;

use crate::provide::*;

const INF: usize = usize::MAX;

struct Element(NodeID, usize);

impl PartialEq for Element {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}

impl Eq for Element {}

impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.1.partial_cmp(&other.1)
    }
}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub struct KthDijkstra<'a, S> {
    storage: &'a S,
}

impl<'a, S> KthDijkstra<'a, S>
where
    S: Node + Edge,
{
    pub fn init(storage: &'a S) -> Self {
        KthDijkstra { storage }
    }

    pub fn exec(
        &mut self,
        start: NodeID,
        target: Option<NodeID>,
        cost_of: impl Fn(NodeID, NodeID) -> usize,
        k: usize, // FIXME: Panic on k < 1?
    ) {
        let node_count = self.storage.node_count();
        let idmap = self.storage.idmap();

        let mut count = vec![0; node_count];
        let mut costs = vec![vec![INF; node_count]; k];
        let mut used_edge = vec![vec![None; node_count]; k];
        let mut heap = BinaryHeap::new();

        let start_vid = idmap[start];

        costs[0][start_vid] = 0;
        heap.push(Element(start, 0));

        while let Some(Element(node, cost)) = heap.pop() {
            let node_vid = idmap[node];

            count[node_vid] += 1;

            let curr_count = count[node_vid];

            if curr_count > k {
                continue;
            }

            if target == Some(node) && curr_count == k {
                break;
            }

            for dst in self.storage.outgoing(node) {
                let dst_vid = idmap[dst];

                let new_cost = cost + cost_of(node, dst);
                let old_cost = costs[curr_count - 1][dst_vid];

                if new_cost < old_cost {
                    costs[curr_count][dst_vid] = new_cost;
                    used_edge[curr_count][dst_vid] = Some((node, dst));
                    heap.push(Element(dst, new_cost));
                }
            }
        }
    }
}
