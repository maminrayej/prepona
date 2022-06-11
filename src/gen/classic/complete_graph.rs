use crate::gen::Generator;
use crate::provide::{AddEdgeProvider, AddNodeProvider, Direction, EmptyStorage, NodeId};

#[derive(Debug, Clone, Copy)]
pub struct CompleteGraph {
    node_count: usize,
}

impl CompleteGraph {
    pub fn init(node_count: usize) -> CompleteGraph {
        if node_count < 3 {
            panic!("Can not generate a complete graph with less than 3 nodes: ${node_count} < 3")
        }

        CompleteGraph { node_count }
    }
}

impl<S> Generator<S> for CompleteGraph
where
    S: EmptyStorage + AddNodeProvider + AddEdgeProvider,
{
    fn generate_into(&self, storage: &mut S, start_node: NodeId) -> NodeId {
        for node in start_node.range(self.node_count) {
            storage.add_node(node);
        }

        for src_node in start_node.range(self.node_count) {
            for dst_node in start_node.range(self.node_count) {
                if src_node < dst_node || (src_node != dst_node && S::Dir::is_directed()) {
                    storage.add_edge(src_node, dst_node);
                }
            }
        }

        start_node + self.node_count
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
            .map(|node| storage.neighbors(node).count())
            .all(|neighbors_count| neighbors_count == generator.node_count - 1));
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
                    assert!(storage.neighbors(node1).contains(&node2));
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
            .map(|node| storage.neighbors(node).count())
            .all(|neighbors_count| neighbors_count == generator.node_count - 1));
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
                    assert!(storage.neighbors(node1).contains(&node2));
                    assert!(storage.predecessors(node1).contains(&node2));
                    assert!(storage.successors(node1).contains(&node2));
                }
            }
        }
    }
}
