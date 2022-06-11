use itertools::Itertools;

use crate::gen::Generator;
use crate::provide::{AddEdgeProvider, AddNodeProvider, EmptyStorage, NodeId};

#[derive(Debug, Clone, Copy)]
pub struct PathGraph {
    node_count: usize,
}

impl PathGraph {
    pub fn init(node_count: usize) -> PathGraph {
        if node_count < 3 {
            panic!("Can not generate a path graph with less than 3 nodes: ${node_count} < 3")
        }

        PathGraph { node_count }
    }
}

impl<S> Generator<S> for PathGraph
where
    S: EmptyStorage + AddNodeProvider + AddEdgeProvider,
{
    fn generate_into(&self, storage: &mut S, start_node: NodeId) -> NodeId {
        for node in start_node.range(self.node_count) {
            storage.add_node(node);
        }

        for (src_node, dst_node) in start_node.range(self.node_count).tuple_windows() {
            storage.add_edge(src_node, dst_node);
        }

        start_node + self.node_count
    }
}

#[cfg(test)]
mod arbitrary {
    use quickcheck::Arbitrary;

    use super::PathGraph;

    impl Arbitrary for PathGraph {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            PathGraph {
                node_count: usize::arbitrary(g) % 32 + 3,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;

    use crate::gen::Generator;
    use crate::provide::{Directed, EdgeProvider, NodeProvider, Undirected};
    use crate::storage::AdjMap;

    use super::PathGraph;

    #[quickcheck]
    fn path_graph_directed(generator: PathGraph) {
        let storage: AdjMap<Directed> = generator.generate();

        assert_eq!(storage.node_count(), generator.node_count);
        assert_eq!(storage.edge_count(), generator.node_count - 1);

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
            1
        );
        assert_eq!(
            storage
                .nodes()
                .filter(|node| storage.successors(*node).count() == 1)
                .count(),
            generator.node_count - 1
        );
        assert_eq!(
            storage
                .nodes()
                .filter(|node| storage.successors(*node).count() == 1)
                .count(),
            generator.node_count - 1
        );
        assert_eq!(
            storage
                .nodes()
                .filter(|node| storage.neighbors(*node).count() == 1)
                .count(),
            2
        );
        assert_eq!(
            storage
                .nodes()
                .filter(|node| storage.neighbors(*node).count() == 2)
                .count(),
            generator.node_count - 2
        );
    }

    #[quickcheck]
    fn path_graph_undirected(generator: PathGraph) {
        let storage: AdjMap<Undirected> = generator.generate();

        assert_eq!(storage.node_count(), generator.node_count);
        assert_eq!(storage.edge_count(), generator.node_count - 1);

        assert_eq!(
            storage
                .nodes()
                .filter(|node| storage.neighbors(*node).count() == 1)
                .count(),
            2
        );
        assert_eq!(
            storage
                .nodes()
                .filter(|node| storage.neighbors(*node).count() == 2)
                .count(),
            generator.node_count - 2
        );

        assert_eq!(
            storage
                .nodes()
                .filter(|node| storage.successors(*node).count() == 1)
                .count(),
            2
        );
        assert_eq!(
            storage
                .nodes()
                .filter(|node| storage.successors(*node).count() == 2)
                .count(),
            generator.node_count - 2
        );

        assert_eq!(
            storage
                .nodes()
                .filter(|node| storage.predecessors(*node).count() == 1)
                .count(),
            2
        );
        assert_eq!(
            storage
                .nodes()
                .filter(|node| storage.predecessors(*node).count() == 2)
                .count(),
            generator.node_count - 2
        );

    }
}
