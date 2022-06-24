use crate::gen::Generator;
use crate::provide::{AddEdgeProvider, AddNodeProvider, Direction, EmptyStorage, NodeId};

use super::EmptyGraph;

#[derive(Debug, Clone, Copy)]
pub struct CompleteGraph {
    node_count: usize,
}

impl CompleteGraph {
    pub fn init(node_count: usize) -> CompleteGraph {
        if node_count < 3 {
            panic!("Can not generate a complete graph with less than 3 nodes: {node_count} < 3")
        }

        CompleteGraph { node_count }
    }
}

impl<S> Generator<S> for CompleteGraph
where
    S: EmptyStorage + AddNodeProvider + AddEdgeProvider,
{
    fn generate_into(&self, storage: &mut S, start_node: NodeId) -> NodeId {
        let next_node = EmptyGraph::init(self.node_count).generate_into(storage, start_node);

        for src_node in start_node.until(next_node) {
            for dst_node in start_node.until(next_node) {
                if src_node < dst_node || (src_node != dst_node && S::Dir::is_directed()) {
                    storage.add_edge(src_node, dst_node);
                }
            }
        }

        next_node
    }
}

#[cfg(test)]
mod arbitrary {
    use quickcheck::Arbitrary;

    use super::CompleteGraph;

    impl Arbitrary for CompleteGraph {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            CompleteGraph {
                node_count: usize::arbitrary(g) % 32 + 3,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use quickcheck_macros::quickcheck;

    use crate::gen::Generator;
    use crate::provide::{Directed, EdgeProvider, NodeProvider, Undirected};
    use crate::storage::AdjMap;

    use super::CompleteGraph;

    #[test]
    #[should_panic(expected = "Can not generate a complete graph with less than 3 nodes: 0 < 3")]
    fn complete_graph_of_size_zero() {
        let _: AdjMap<Undirected> = CompleteGraph::init(0).generate();
    }

    #[test]
    #[should_panic(expected = "Can not generate a complete graph with less than 3 nodes: 1 < 3")]
    fn complete_graph_of_size_one() {
        let _: AdjMap<Undirected> = CompleteGraph::init(1).generate();
    }

    #[test]
    #[should_panic(expected = "Can not generate a complete graph with less than 3 nodes: 2 < 3")]
    fn complete_graph_of_size_two() {
        let _: AdjMap<Undirected> = CompleteGraph::init(2).generate();
    }

    #[quickcheck]
    fn complete_graph_directed(generator: CompleteGraph) {
        let storage: AdjMap<Directed> = generator.generate();

        assert_eq!(storage.node_count(), generator.node_count);
        assert_eq!(
            storage.edge_count(),
            generator.node_count * (generator.node_count - 1)
        );
        assert!(storage
            .nodes()
            .map(|node| storage.successors(node).count())
            .all(|neighbors_count| neighbors_count == generator.node_count - 1));
        assert!(storage
            .nodes()
            .map(|node| storage.predecessors(node).count())
            .all(|neighbors_count| neighbors_count == generator.node_count - 1));

        for node1 in storage.nodes() {
            for node2 in storage.nodes() {
                if node1 != node2 {
                    assert!(storage.predecessors(node1).contains(&node2));
                    assert!(storage.successors(node1).contains(&node2));
                }
            }
        }
    }

    #[quickcheck]
    fn complete_graph_undirected(generator: CompleteGraph) {
        let storage: AdjMap<Undirected> = generator.generate();

        assert_eq!(storage.node_count(), generator.node_count);
        assert_eq!(
            storage.edge_count(),
            generator.node_count * (generator.node_count - 1) / 2
        );
        assert!(storage
            .nodes()
            .map(|node| storage.successors(node).count())
            .all(|neighbors_count| neighbors_count == generator.node_count - 1));
        assert!(storage
            .nodes()
            .map(|node| storage.predecessors(node).count())
            .all(|neighbors_count| neighbors_count == generator.node_count - 1));

        for node1 in storage.nodes() {
            for node2 in storage.nodes() {
                if node1 != node2 {
                    assert!(storage.predecessors(node1).contains(&node2));
                    assert!(storage.successors(node1).contains(&node2));
                }
            }
        }
    }
}
