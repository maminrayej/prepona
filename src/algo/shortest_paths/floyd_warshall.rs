use std::collections::HashMap;

use anyhow::Result;

use crate::algo::AlgoError;
use crate::provide::{Directed, EdgeProvider, NodeId, NodeIdMapProvider};

const INFINITE: isize = isize::MAX;

pub struct FloydWarshall<'a, G>
where
    G: EdgeProvider<Dir = Directed> + NodeIdMapProvider,
{
    graph: &'a G,
    id_map: G::NodeIdMap,
}

impl<'a, G> FloydWarshall<'a, G>
where
    G: EdgeProvider<Dir = Directed> + NodeIdMapProvider,
{
    pub fn new(graph: &'a G) -> Self {
        FloydWarshall {
            graph,
            id_map: graph.id_map(),
        }
    }

    pub fn execute(
        &mut self,
        cost_of: impl Fn(NodeId, NodeId) -> isize,
    ) -> Result<HashMap<(NodeId, NodeId), isize>> {
        let node_count = self.graph.node_count();

        let mut dist = vec![vec![INFINITE; node_count]; node_count];

        for node in self.graph.nodes() {
            let n_vid = self.id_map[node];

            dist[n_vid][n_vid] = 0;
        }

        for node in self.graph.nodes() {
            let n_vid = self.id_map[node];

            for successor in self.graph.successors(node) {
                let s_vid = self.id_map[successor];

                dist[n_vid][s_vid] = cost_of(node, successor);
            }
        }

        for k in 0..node_count {
            for node1 in self.graph.nodes() {
                for node2 in self.graph.nodes() {
                    let n1_vid = self.id_map[node1];
                    let n2_vid = self.id_map[node2];

                    let cost_through_k = dist[n1_vid][k].saturating_add(dist[k][n2_vid]);
                    let direct_cost = dist[n1_vid][n2_vid];

                    if cost_through_k < direct_cost {
                        dist[n1_vid][n2_vid] = cost_through_k;
                    }
                }

                // Check for negative cycles.
                for node in self.graph.nodes() {
                    let n_vid = self.id_map[node];

                    if dist[n_vid][n_vid] < 0 {
                        return Err(AlgoError::NegativeCycleDetected.into());
                    }
                }
            }
        }

        let mut distance_map = HashMap::new();

        for node1 in self.graph.nodes() {
            for node2 in self.graph.nodes() {
                let n1_vid = self.id_map[node1];
                let n2_vid = self.id_map[node2];

                if dist[n1_vid][n2_vid] != INFINITE {
                    distance_map.insert((node1, node2), dist[n1_vid][n2_vid]);
                }
            }
        }

        Ok(distance_map)
    }
}

#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;

    use crate::gen::{CompleteGraph, Generator, PathGraph};
    use crate::provide::{Directed, NodeProvider};
    use crate::storage::AdjMap;

    use super::FloydWarshall;

    #[quickcheck]
    fn floyd_warshall_on_complete_graph(generator: CompleteGraph) {
        let graph: AdjMap<Directed> = generator.generate();

        let mut floyd_warshall = FloydWarshall::new(&graph);

        let dist_map = floyd_warshall.execute(|_, _| 1).unwrap();

        for node in graph.nodes() {
            for other in graph.nodes() {
                if node == other {
                    assert_eq!(dist_map[&(node, other)], 0);
                } else {
                    assert_eq!(dist_map[&(node, other)], 1);
                }
            }
        }
    }

    #[quickcheck]
    fn floyd_warshall_on_path_graph(generator: PathGraph) {
        if generator.node_count > 0 {
            let graph: AdjMap<Directed> = generator.generate();

            let mut floyd_warshall = FloydWarshall::new(&graph);

            let dist_map = floyd_warshall.execute(|_, _| 1).unwrap();

            for node in graph.nodes() {
                for other in graph.nodes() {
                    if node <= other {
                        assert_eq!(
                            dist_map[&(node, other)],
                            node.inner().abs_diff(other.inner()) as isize
                        );
                    }
                }
            }
        }
    }
}
