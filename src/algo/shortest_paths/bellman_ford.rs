use anyhow::Result;
use indexmap::IndexMap;

use crate::algo::AlgoError;
use crate::provide::{Directed, EdgeProvider, NodeId, NodeIdMapProvider};
use crate::view::GenericView;

const INFINITE: isize = isize::MAX;

pub struct BellmanFord<'a, G>
where
    G: EdgeProvider<Dir = Directed> + NodeIdMapProvider,
{
    graph: &'a G,
    id_map: G::NodeIdMap,
    src_node: NodeId,

    costs: Vec<isize>,
    parent: IndexMap<NodeId, (NodeId, isize)>,
}

impl<'a, G> BellmanFord<'a, G>
where
    G: EdgeProvider<Dir = Directed> + NodeIdMapProvider,
{
    pub fn new(graph: &'a G, src_node: NodeId) -> Self {
        BellmanFord {
            graph,
            id_map: graph.id_map(),
            src_node,
            costs: vec![INFINITE; graph.node_count()],
            parent: IndexMap::new(),
        }
    }

    pub fn execute(
        &mut self,
        cost_of: impl Fn(NodeId, NodeId) -> isize,
    ) -> Result<IndexMap<NodeId, isize>> {
        self.costs[self.id_map[self.src_node]] = 0;

        for _ in 1..self.graph.node_count() {
            for (src_node, dst_node) in self.graph.edges() {
                let src_vid = self.id_map[src_node];
                let dst_vid = self.id_map[dst_node];

                let s_cost = self.costs[src_vid];
                let d_cost = self.costs[dst_vid];

                let new_cost = s_cost.saturating_add(cost_of(src_node, dst_node));
                if new_cost < d_cost {
                    self.costs[dst_vid] = new_cost;
                    self.parent.insert(dst_node, (src_node, new_cost));
                }
            }
        }

        for (src_node, dst_node) in self.graph.edges() {
            let s_vid = self.id_map[src_node];
            let d_vid = self.id_map[dst_node];

            let s_cost = self.costs[s_vid];
            let d_cost = self.costs[d_vid];

            let new_cost = s_cost.saturating_add(cost_of(src_node, dst_node));
            if new_cost < d_cost {
                return Err(AlgoError::NegativeCycleDetected.into());
            }
        }

        Ok(self
            .parent
            .iter()
            .map(|(node, (_, cost))| (*node, *cost))
            .chain(Some((self.src_node, 0)).into_iter())
            .collect())
    }

    pub fn reconstruct(self) -> GenericView<'a, G> {
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
    use quickcheck_macros::quickcheck;

    use crate::gen::{CompleteGraph, Generator, PathGraph};
    use crate::provide::{Directed, NodeProvider};
    use crate::storage::AdjMap;

    use super::BellmanFord;

    #[quickcheck]
    fn bellman_ford_on_complete_graph(generator: CompleteGraph) {
        let graph: AdjMap<Directed> = generator.generate();

        let mut bellman_ford = BellmanFord::new(&graph, 0.into());

        let dist_map = bellman_ford.execute(|_, _| 1).unwrap();

        assert_eq!(dist_map.keys().count(), graph.node_count());
        assert_eq!(
            dist_map.values().filter(|dist| **dist == 1).count(),
            graph.node_count() - 1
        );
        assert_eq!(dist_map.values().filter(|dist| **dist == 0).count(), 1);
    }

    #[quickcheck]
    fn bellman_ford_on_path_graph(generator: PathGraph) {
        if generator.node_count > 0 {
            let graph: AdjMap<Directed> = generator.generate();

            let mut bellman_ford = BellmanFord::new(&graph, 0.into());

            let dist_map = bellman_ford.execute(|_, _| 1).unwrap();

            assert!(dist_map
                .iter()
                .all(|(node, cost)| node.inner() == (*cost as usize)));
        }
    }
}
