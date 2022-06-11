use core::panic;

use crate::gen::Generator;
use crate::provide::{AddEdgeProvider, AddNodeProvider, EmptyStorage, NodeId};

#[derive(Debug, Clone, Copy)]
pub struct StarGraph {
    node_count: usize,
}

impl StarGraph {
    pub fn init(node_count: usize) -> StarGraph {
        if node_count < 4 {
            panic!("Can not form a star graph with less than 4 nodes: {node_count} < 4")
        }

        StarGraph { node_count }
    }
}

impl<S> Generator<S> for StarGraph
where
    S: EmptyStorage + AddNodeProvider + AddEdgeProvider,
{
    fn generate_into(&self, storage: &mut S, start_node: NodeId) -> NodeId {
        for node in start_node.range(self.node_count) {
            storage.add_node(node);
        }

        for other_node in start_node.range(self.node_count).skip(1) {
            storage.add_edge(start_node, other_node)
        }

        start_node + self.node_count
    }
}

#[cfg(test)]
mod arbitrary {
    use quickcheck::Arbitrary;

    use super::StarGraph;

    impl Arbitrary for StarGraph {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            StarGraph {
                node_count: usize::arbitrary(g) % 32 + 4,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;

    use crate::gen::Generator;
    use crate::provide::{Directed, NodeProvider, Undirected};
    use crate::storage::AdjMap;

    use super::StarGraph;

    #[quickcheck]
    fn empty_graph_directed(generator: StarGraph) {
        let storage: AdjMap<Directed> = generator.generate();

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
                .filter(|node| storage.predecessors(*node).count() == 0)
                .count(),
            1
        );

        assert_eq!(
            storage
                .nodes()
                .filter(|node| storage.successors(*node).count() == 0)
                .count(),
            generator.node_count - 1
        );

        assert_eq!(
            storage
                .nodes()
                .filter(|node| storage.predecessors(*node).count() == 1)
                .count(),
            generator.node_count - 1
        );
    }

    #[quickcheck]
    fn empty_graph_undirected(generator: StarGraph) {
        let storage: AdjMap<Undirected> = generator.generate();

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
                .filter(|node| storage.successors(*node).count() == 1)
                .count(),
            generator.node_count - 1
        );
    }
}
