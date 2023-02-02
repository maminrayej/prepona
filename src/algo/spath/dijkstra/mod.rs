mod view;
pub use view::DijkstraSPT;

use std::collections::BinaryHeap;

use crate::give::*;
use crate::view::{EdgeSetSelector, NodeSetSelector, View};

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

pub struct Dijkstra<'a, S> {
    storage: &'a S,
}

impl<'a, S> Dijkstra<'a, S>
where
    S: Edge,
{
    pub fn init(storage: &'a S) -> Self {
        Dijkstra { storage }
    }

    pub fn exec(
        &mut self,
        start: NodeID,
        target: Option<NodeID>,
        cost_of: impl Fn(NodeID, NodeID) -> usize,
    ) -> DijkstraSPT<'a, S> {
        let node_count = self.storage.node_count();

        let idmap = self.storage.idmap();

        let mut visited = vec![false; node_count];
        let mut costs = vec![INF; node_count];
        let mut used_edge = vec![None; node_count];
        let mut heap = BinaryHeap::new();

        let start_vid = idmap[start];

        costs[start_vid] = 0;
        heap.push(Element(start, 0));

        while let Some(Element(node, cost)) = heap.pop() {
            let node_vid = idmap[node];

            if visited[node_vid] {
                continue;
            }

            if target == Some(node) {
                break;
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
                    heap.push(Element(dst, new_cost));
                }
            }

            visited[node_vid] = true;
        }

        let node_set = visited
            .into_iter()
            .enumerate()
            .filter_map(|(i, visited)| if visited { Some(idmap[i]) } else { None })
            .collect();

        let edge_set = used_edge.into_iter().filter_map(|e| e).collect();

        let cost_map = costs
            .into_iter()
            .enumerate()
            .map(|(i, cost)| (idmap[i], cost))
            .collect();

        DijkstraSPT {
            view: View::new(
                self.storage,
                NodeSetSelector::new(node_set),
                EdgeSetSelector::new(edge_set),
            ),
            cost: cost_map,
        }
    }
}
