use crate::gen::Generator;
use crate::provide::{AddEdgeProvider, AddNodeProvider, EmptyStorage, NodeId};

#[derive(Debug, Clone, Copy)]
pub struct NullGraph;

impl NullGraph {
    pub fn init() -> NullGraph {
        NullGraph
    }
}

impl<S> Generator<S> for NullGraph
where
    S: EmptyStorage + AddNodeProvider + AddEdgeProvider,
{
    fn generate_into(&self, _: &mut S, start_node: NodeId) -> NodeId {
        start_node
    }
}

#[cfg(test)]
mod arbitrary {
    use quickcheck::Arbitrary;

    use super::NullGraph;

    impl Arbitrary for NullGraph {
        fn arbitrary(_: &mut quickcheck::Gen) -> Self {
            NullGraph
        }
    }
}

#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;

    use crate::gen::Generator;
    use crate::provide::{Directed, EdgeProvider, NodeProvider, Undirected};
    use crate::storage::AdjMap;

    use super::NullGraph;

    #[quickcheck]
    fn null_graph_directed(generator: NullGraph) {
        let storage: AdjMap<Directed> = generator.generate();

        assert_eq!(storage.node_count(), 0);
        assert_eq!(storage.edge_count(), 0);
    }

    #[quickcheck]
    fn null_graph_undirected(generator: NullGraph) {
        let storage: AdjMap<Undirected> = generator.generate();

        assert_eq!(storage.node_count(), 0);
        assert_eq!(storage.edge_count(), 0);
    }
}
