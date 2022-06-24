use itertools::Itertools;

use crate::gen::Generator;
use crate::provide::{AddEdgeProvider, AddNodeProvider, EmptyStorage, NodeId};

use super::EmptyGraph;

#[derive(Debug, Clone, Copy)]
pub struct PathGraph {
    node_count: usize,
}

impl PathGraph {
    pub fn init(node_count: usize) -> PathGraph {
        PathGraph { node_count }
    }
}

impl<S> Generator<S> for PathGraph
where
    S: EmptyStorage + AddNodeProvider + AddEdgeProvider,
{
    fn generate_into(&self, storage: &mut S, start_node: NodeId) -> NodeId {
        let next_node = EmptyGraph::init(self.node_count).generate_into(storage, start_node);

        for (src_node, dst_node) in start_node.until(next_node).tuple_windows() {
            storage.add_edge(src_node, dst_node);
        }

        next_node
    }
}

#[cfg(test)]
mod arbitrary {
    use quickcheck::Arbitrary;

    use super::PathGraph;

    impl Arbitrary for PathGraph {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            PathGraph {
                node_count: usize::arbitrary(g) % 32,
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

        if generator.node_count == 0 {
            assert_eq!(storage.node_count(), 0);
            assert_eq!(storage.edge_count(), 0);
        } else if generator.node_count == 1 {
            assert_eq!(storage.node_count(), 1);
            assert_eq!(storage.edge_count(), 0);
        } else {
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
                    .filter(|node| storage.predecessors(*node).count() == 1)
                    .count(),
                generator.node_count - 1
            );
        }
    }

    #[quickcheck]
    fn path_graph_undirected(generator: PathGraph) {
        let storage: AdjMap<Undirected> = generator.generate();

        if generator.node_count == 0 {
            assert_eq!(storage.node_count(), 0);
            assert_eq!(storage.edge_count(), 0);
        } else if generator.node_count == 1 {
            assert_eq!(storage.node_count(), 1);
            assert_eq!(storage.edge_count(), 0);
        } else {
            assert_eq!(storage.node_count(), generator.node_count);
            assert_eq!(storage.edge_count(), generator.node_count - 1);

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
}
