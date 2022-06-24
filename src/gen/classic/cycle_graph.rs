use itertools::Itertools;

use crate::gen::Generator;
use crate::provide::{AddEdgeProvider, AddNodeProvider, EmptyStorage, NodeId};

use super::EmptyGraph;

#[derive(Debug, Clone, Copy)]
pub struct CycleGraph {
    node_count: usize,
}

impl CycleGraph {
    pub fn init(node_count: usize) -> CycleGraph {
        if node_count < 3 {
            panic!("Can not form a cycle graph with less than 3 nodes: {node_count} < 3")
        }

        CycleGraph { node_count }
    }
}

impl<S> Generator<S> for CycleGraph
where
    S: EmptyStorage + AddNodeProvider + AddEdgeProvider,
{
    fn generate_into(&self, storage: &mut S, start_node: NodeId) -> NodeId {
        let next_node = EmptyGraph::init(self.node_count).generate_into(storage, start_node);

        for (src_node, dst_node) in start_node.until(next_node).tuple_windows() {
            storage.add_edge(src_node, dst_node);
        }

        storage.add_edge(next_node - 1, start_node);

        next_node
    }
}

#[cfg(test)]
mod arbitrary {
    use quickcheck::Arbitrary;

    use super::CycleGraph;

    impl Arbitrary for CycleGraph {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            CycleGraph {
                node_count: usize::arbitrary(g) % 32 + 3,
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

    use super::CycleGraph;

    #[test]
    #[should_panic(expected = "Can not form a cycle graph with less than 3 nodes: 0 < 3")]
    fn cycle_graph_of_size_zero() {
        let _: AdjMap<Undirected> = CycleGraph::init(0).generate();
    }

    #[test]
    #[should_panic(expected = "Can not form a cycle graph with less than 3 nodes: 1 < 3")]
    fn cycle_graph_of_size_one() {
        let _: AdjMap<Undirected> = CycleGraph::init(1).generate();
    }

    #[test]
    #[should_panic(expected = "Can not form a cycle graph with less than 3 nodes: 2 < 3")]
    fn cycle_graph_of_size_two() {
        let _: AdjMap<Undirected> = CycleGraph::init(2).generate();
    }

    #[quickcheck]
    fn cycle_graph_directed(generator: CycleGraph) {
        let storage: AdjMap<Directed> = generator.generate();
        assert!(storage
            .nodes()
            .all(|node| storage.successors(node).count() == 1));
        assert!(storage
            .nodes()
            .all(|node| storage.predecessors(node).count() == 1));
    }

    #[quickcheck]
    fn cycle_graph_undirected(generator: CycleGraph) {
        let storage: AdjMap<Undirected> = generator.generate();

        assert!(storage
            .nodes()
            .all(|node| storage.successors(node).count() == 2));
    }
}
