use crate::gen::Generator;
use crate::provide::{AddEdgeProvider, AddNodeProvider, EmptyStorage, NodeId, Undirected};

use super::CycleGraph;

#[derive(Debug, Clone, Copy)]
pub struct WheelGraph {
    node_count: usize,
}

impl WheelGraph {
    pub fn init(node_count: usize) -> WheelGraph {
        if node_count < 4 {
            panic!("Can not form a wheel graph with less than 4 nodes: {node_count} < 4")
        }

        WheelGraph { node_count }
    }
}

impl<S> Generator<S> for WheelGraph
where
    S: EmptyStorage<Dir = Undirected> + AddNodeProvider + AddEdgeProvider,
{
    fn generate_into(&self, storage: &mut S, start_node: NodeId) -> NodeId {
        let center_node = CycleGraph::init(self.node_count - 1).generate_into(storage, start_node);

        storage.add_node(center_node);

        for other_node in start_node.until(center_node) {
            storage.add_edge(center_node, other_node);
        }

        center_node + 1
    }
}

#[cfg(test)]
mod arbitrary {
    use quickcheck::Arbitrary;

    use super::WheelGraph;

    impl Arbitrary for WheelGraph {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            WheelGraph {
                node_count: usize::arbitrary(g) % 32 + 4,
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

    use super::WheelGraph;

    #[test]
    #[should_panic(expected = "Can not form a wheel graph with less than 4 nodes: 0 < 4")]
    fn wheel_graph_of_size_zero() {
        let _: AdjMap<Undirected> = WheelGraph::init(0).generate();
    }

    #[test]
    #[should_panic(expected = "Can not form a wheel graph with less than 4 nodes: 1 < 4")]
    fn wheel_graph_of_size_one() {
        let _: AdjMap<Undirected> = WheelGraph::init(1).generate();
    }

    #[test]
    #[should_panic(expected = "Can not form a wheel graph with less than 4 nodes: 2 < 4")]
    fn wheel_graph_of_size_two() {
        let _: AdjMap<Undirected> = WheelGraph::init(2).generate();
    }

    #[test]
    #[should_panic(expected = "Can not form a wheel graph with less than 4 nodes: 3 < 4")]
    fn wheel_graph_of_size_three() {
        let _: AdjMap<Undirected> = WheelGraph::init(3).generate();
    }

    #[quickcheck]
    fn wheel_graph_undirected(generator: WheelGraph) {
        let storage: AdjMap<Undirected> = generator.generate();

        if generator.node_count == 4 {
            assert!(storage
                .nodes()
                .all(|node| storage.successors(node).count() == 3));
        } else {
            assert_eq!(
                storage
                    .nodes()
                    .filter(|node| storage.successors(*node).count() == generator.node_count - 1)
                    .count(),
                1
            );
            assert_eq!(
                storage
                    .nodes()
                    .filter(|node| storage.successors(*node).count() == 3)
                    .count(),
                generator.node_count - 1
            );
        }
    }
}
