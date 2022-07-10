use std::collections::{BinaryHeap, HashMap};

use indexmap::IndexMap;

use crate::provide::{EdgeProvider, NodeId, NodeIdMapProvider};
use crate::view::GenericView;

const INFINITE: usize = usize::MAX;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Record {
    node_vid: usize,
    cost: usize,
}

impl PartialOrd for Record {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.cost.partial_cmp(&other.cost)
    }
}

impl Ord for Record {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub struct Dijkstra<'a, G>
where
    G: EdgeProvider + NodeIdMapProvider,
{
    graph: &'a G,
    id_map: G::NodeIdMap,

    src_node: NodeId,
    dst_node: Option<NodeId>,

    visited: Vec<bool>,
    costs: Vec<usize>,
    parent: IndexMap<NodeId, (NodeId, usize)>,
}

impl<'a, G> Dijkstra<'a, G>
where
    G: EdgeProvider + NodeIdMapProvider,
{
    pub fn new(graph: &'a G, src_node: NodeId, dst_node: Option<NodeId>) -> Self {
        Dijkstra {
            graph,
            id_map: graph.id_map(),
            src_node,
            dst_node,
            visited: vec![false; graph.node_count()],
            costs: vec![INFINITE; graph.node_count()],
            parent: IndexMap::new(),
        }
    }

    pub fn execute(&mut self, cost_of: impl Fn(NodeId, NodeId) -> usize) -> HashMap<NodeId, usize> {
        let mut heap = BinaryHeap::new();

        let src_vid = self.id_map[self.src_node];

        self.costs[src_vid] = 0;
        self.parent.insert(self.src_node, (self.src_node, 0));

        heap.push(Record {
            node_vid: src_vid,
            cost: 0,
        });

        while let Some(Record { node_vid, cost }) = heap.pop() {
            if self.visited[node_vid] {
                continue;
            }

            let node = self.id_map[node_vid];

            if Some(node) == self.dst_node {
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
                    self.parent.insert(successor, (node, new_cost));
                    heap.push(Record {
                        node_vid: s_vid,
                        cost: new_cost,
                    });
                }
            }

            self.visited[node_vid] = true;
        }

        self.parent
            .iter()
            .map(|(node, (_, cost))| (*node, *cost))
            .collect()
    }

    pub fn reconstrcut(self) -> GenericView<'a, G> {
        // FIXME: order of node and predecessor is not respected in this initialization.
        // TODO: Add unit tests to test this method.
        GenericView::new(
            self.graph,
            self.parent.keys().copied(),
            self.parent
                .iter()
                .map(|(node, (predecessor, _))| (*predecessor, *node)),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::gen::{CompleteGraph, Generator, PathGraph};
    use crate::provide::{Directed, Direction, NodeProvider, Undirected};
    use crate::storage::AdjMap;

    use super::Dijkstra;

    #[test]
    fn dijkstra_on_complete_graph() {
        fn test<Dir: Direction>(generator: CompleteGraph) -> bool {
            let graph: AdjMap<Dir> = generator.generate();

            let mut dijkstra = Dijkstra::new(&graph, 0.into(), None);

            let dist_map = dijkstra.execute(|_, _| 1);

            assert_eq!(dist_map.keys().count(), graph.node_count());
            assert_eq!(
                dist_map.values().filter(|dist| **dist == 1).count(),
                graph.node_count() - 1
            );
            assert_eq!(dist_map.values().filter(|dist| **dist == 0).count(), 1);

            true
        }

        quickcheck::quickcheck(test::<Undirected> as fn(CompleteGraph) -> bool);
        quickcheck::quickcheck(test::<Directed> as fn(CompleteGraph) -> bool);
    }

    #[test]
    fn dijkstra_on_path_graph() {
        fn test<Dir: Direction>(generator: PathGraph) -> bool {
            if generator.node_count > 0 {
                let graph: AdjMap<Undirected> = generator.generate();

                let mut dijkstra = Dijkstra::new(&graph, 0.into(), None);

                let dist_map = dijkstra.execute(|_, _| 1);

                assert!(dist_map.iter().all(|(node, cost)| node.inner() == *cost));
            }

            true
        }

        quickcheck::quickcheck(test::<Undirected> as fn(PathGraph) -> bool);
        quickcheck::quickcheck(test::<Directed> as fn(PathGraph) -> bool);
    }
}
