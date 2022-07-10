use std::collections::BinaryHeap;

use crate::provide::{EdgeProvider, NodeId, NodeIdMapProvider};

const INFINITE: usize = usize::MAX;

#[derive(Debug, PartialEq, Eq)]
struct Record {
    node_vid: usize,
    cost: usize,
    estimate: usize,
}

impl PartialOrd for Record {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.estimate.partial_cmp(&other.estimate)
    }
}

impl Ord for Record {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub struct AStar<'a, G>
where
    G: EdgeProvider + NodeIdMapProvider,
{
    graph: &'a G,
    id_map: G::NodeIdMap,

    src_node: NodeId,
    dst_node: NodeId,

    visited: Vec<bool>,
    costs: Vec<usize>,
}

impl<'a, G> AStar<'a, G>
where
    G: EdgeProvider + NodeIdMapProvider,
{
    pub fn new(graph: &'a G, src_node: NodeId, dst_node: NodeId) -> Self {
        AStar {
            graph,
            id_map: graph.id_map(),
            src_node,
            dst_node,
            visited: vec![false; graph.node_count()],
            costs: vec![INFINITE; graph.node_count()],
        }
    }

    pub fn execute(
        &mut self,
        cost_of: impl Fn(NodeId, NodeId) -> usize,
        estimate_of: impl Fn(NodeId) -> usize,
    ) -> usize {
        let mut heap = BinaryHeap::new();

        let src_vid = self.id_map[self.src_node];

        self.costs[src_vid] = 0;

        heap.push(Record {
            node_vid: src_vid,
            cost: 0,
            estimate: estimate_of(self.src_node),
        });

        while let Some(Record { node_vid, cost, .. }) = heap.pop() {
            if self.visited[node_vid] {
                continue;
            }

            let node = self.id_map[node_vid];

            if node == self.dst_node {
                break;
            }

            for successor in self.graph.successors(node) {
                let s_vid = self.id_map[successor];

                if self.visited[s_vid] {
                    continue;
                }

                let new_cost = cost.saturating_add(cost_of(node, successor));
                let old_cost = self.costs[s_vid];

                if new_cost < old_cost {
                    self.costs[s_vid] = new_cost;
                    heap.push(Record {
                        node_vid: s_vid,
                        cost: new_cost,
                        estimate: new_cost.saturating_add(estimate_of(successor)),
                    })
                }
            }

            self.visited[node_vid] = true;
        }

        self.costs[self.id_map[self.dst_node]]
    }
}

#[cfg(test)]
mod tests {
    use crate::gen::{CompleteGraph, Generator, PathGraph};
    use crate::provide::{Directed, Direction, Undirected};
    use crate::storage::AdjMap;

    use super::AStar;

    #[test]
    fn astar_on_complete_graph() {
        fn test<Dir: Direction>(generator: CompleteGraph) -> bool {
            let graph: AdjMap<Dir> = generator.generate();

            let mut astar = AStar::new(&graph, 0.into(), 1.into());

            let dist = astar.execute(|_, _| 1, |_| 1);

            assert_eq!(dist, 1);

            true
        }

        quickcheck::quickcheck(test::<Undirected> as fn(CompleteGraph) -> bool);
        quickcheck::quickcheck(test::<Directed> as fn(CompleteGraph) -> bool);
    }

    #[test]
    fn astar_on_path_graph() {
        fn test<Dir: Direction>(generator: PathGraph) -> bool {
            if generator.node_count > 0 {
                let graph: AdjMap<Undirected> = generator.generate();

                let mut astar = AStar::new(&graph, 0.into(), (generator.node_count - 1).into());

                let dist = astar.execute(|_, _| 1, |_| 1);

                assert_eq!(dist, generator.node_count - 1);
            }

            true
        }

        quickcheck::quickcheck(test::<Undirected> as fn(PathGraph) -> bool);
        quickcheck::quickcheck(test::<Directed> as fn(PathGraph) -> bool);
    }
}
