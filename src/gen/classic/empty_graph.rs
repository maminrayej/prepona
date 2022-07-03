use crate::gen::Generator;
use crate::provide::{AddEdgeProvider, AddNodeProvider, EmptyStorage, NodeId};

#[derive(Debug, Clone, Copy)]
pub struct EmptyGraph {
    pub node_count: usize,
}

impl EmptyGraph {
    pub fn init(node_count: usize) -> EmptyGraph {
        EmptyGraph { node_count }
    }
}

impl<S> Generator<S> for EmptyGraph
where
    S: EmptyStorage + AddNodeProvider + AddEdgeProvider,
{
    fn generate_into(&self, storage: &mut S, start_node: NodeId) -> NodeId {
        for node in start_node.range(self.node_count) {
            storage.add_node(node);
        }

        start_node + self.node_count
    }
}

#[cfg(test)]
mod arbitrary {
    use quickcheck::Arbitrary;

    use super::EmptyGraph;

    impl Arbitrary for EmptyGraph {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            EmptyGraph {
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

    use super::EmptyGraph;

    #[quickcheck]
    fn empty_graph_directed(generator: EmptyGraph) {
        let storage: AdjMap<Directed> = generator.generate();

        assert_eq!(storage.node_count(), generator.node_count);
        assert_eq!(storage.edge_count(), 0);
    }

    #[quickcheck]
    fn empty_graph_undirected(generator: EmptyGraph) {
        let storage: AdjMap<Undirected> = generator.generate();

        assert_eq!(storage.node_count(), generator.node_count);
        assert_eq!(storage.edge_count(), 0);
    }
}
