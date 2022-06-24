use crate::gen::Generator;
use crate::provide::{AddEdgeProvider, AddNodeProvider, EmptyStorage, NodeId, Undirected};

use super::PathGraph;

#[derive(Debug, Clone, Copy)]
pub struct LadderGraph {
    side_node_count: usize,
}

impl LadderGraph {
    pub fn init(side_node_count: usize) -> LadderGraph {
        if side_node_count < 1 {
            panic!(
                "Can not form a ladder graph with less than 1 node for each side: {side_node_count} < 1"
            )
        }

        LadderGraph { side_node_count }
    }
}

impl<S> Generator<S> for LadderGraph
where
    S: EmptyStorage<Dir = Undirected> + AddNodeProvider + AddEdgeProvider,
{
    fn generate_into(&self, storage: &mut S, start_node: NodeId) -> NodeId {
        let next_side_start =
            PathGraph::init(self.side_node_count).generate_into(storage, start_node);
        let next_node =
            PathGraph::init(self.side_node_count).generate_into(storage, next_side_start);

        let pairs = start_node
            .until(next_side_start)
            .zip(next_side_start.until(next_node));

        for (src_node, dst_node) in pairs {
            storage.add_edge(src_node, dst_node);
        }

        next_node
    }
}

#[cfg(test)]
mod arbitrary {
    use quickcheck::Arbitrary;

    use super::LadderGraph;

    impl Arbitrary for LadderGraph {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            LadderGraph {
                side_node_count: usize::arbitrary(g) % 32 + 1,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;

    use crate::gen::Generator;
    use crate::provide::{NodeProvider, Undirected};
    use crate::storage::AdjMap;

    use super::LadderGraph;

    #[test]
    #[should_panic(
        expected = "Can not form a ladder graph with less than 1 node for each side: 0 < 1"
    )]
    fn ladder_graph_of_size_zero() {
        let _: AdjMap<Undirected> = LadderGraph::init(0).generate();
    }

    #[quickcheck]
    fn ladder_graph_undirected(generator: LadderGraph) {
        let graph: AdjMap<Undirected> = generator.generate();

        if generator.side_node_count == 1 {
            assert!(graph
                .nodes()
                .all(|node| graph.successors(node).count() == 1));
        } else {
            assert_eq!(
                graph
                    .nodes()
                    .filter(|node| graph.successors(*node).count() == 2)
                    .count(),
                4
            );
            assert_eq!(
                graph
                    .nodes()
                    .filter(|node| graph.successors(*node).count() == 3)
                    .count(),
                (generator.side_node_count - 2) * 2
            );
        }
    }
}
