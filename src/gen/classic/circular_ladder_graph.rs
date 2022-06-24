use crate::gen::Generator;
use crate::provide::{AddEdgeProvider, AddNodeProvider, EmptyStorage, NodeId, Undirected};

use super::CycleGraph;

#[derive(Debug, Clone, Copy)]
pub struct CircularLadderGraph {
    cycle_node_count: usize,
}

impl CircularLadderGraph {
    pub fn init(cycle_node_count: usize) -> CircularLadderGraph {
        if cycle_node_count < 3 {
            panic!("Can not generate a circular ladder graph with a cycle size smaller than 3: {cycle_node_count} < 3")
        }

        CircularLadderGraph { cycle_node_count }
    }
}

impl<S> Generator<S> for CircularLadderGraph
where
    S: EmptyStorage<Dir = Undirected> + AddNodeProvider + AddEdgeProvider,
{
    fn generate_into(&self, storage: &mut S, start_node: NodeId) -> NodeId {
        let start_of_cycle2 =
            CycleGraph::init(self.cycle_node_count).generate_into(storage, start_node);
        let next_node =
            CycleGraph::init(self.cycle_node_count).generate_into(storage, start_of_cycle2);

        let pairs = start_node
            .until(start_of_cycle2)
            .zip(start_of_cycle2.until(next_node));

        for (src_node, dst_node) in pairs {
            storage.add_edge(src_node, dst_node);
        }

        next_node
    }
}

#[cfg(test)]
mod arbitrary {
    use quickcheck::Arbitrary;

    use super::CircularLadderGraph;

    impl Arbitrary for CircularLadderGraph {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            CircularLadderGraph {
                cycle_node_count: usize::arbitrary(g) % 32 + 3,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;

    use crate::gen::Generator;
    use crate::provide::{EdgeProvider, NodeProvider, Undirected};
    use crate::storage::AdjMap;

    use super::CircularLadderGraph;

    #[test]
    #[should_panic(
        expected = "Can not generate a circular ladder graph with a cycle size smaller than 3: 0 < 3"
    )]
    fn circular_ladder_graph_of_size_zero() {
        let _: AdjMap<Undirected> = CircularLadderGraph::init(0).generate();
    }

    #[test]
    #[should_panic(
        expected = "Can not generate a circular ladder graph with a cycle size smaller than 3: 1 < 3"
    )]
    fn circular_ladder_graph_of_size_one() {
        let _: AdjMap<Undirected> = CircularLadderGraph::init(1).generate();
    }

    #[test]
    #[should_panic(
        expected = "Can not generate a circular ladder graph with a cycle size smaller than 3: 2 < 3"
    )]
    fn circular_ladder_graph_of_size_two() {
        let _: AdjMap<Undirected> = CircularLadderGraph::init(2).generate();
    }

    #[quickcheck]
    fn circular_ladder_graph_undirected(generator: CircularLadderGraph) {
        let storage: AdjMap<Undirected> = generator.generate();

        assert_eq!(storage.node_count(), generator.cycle_node_count * 2);
        assert_eq!(storage.edge_count(), generator.cycle_node_count * 3);

        assert!(storage
            .nodes()
            .all(|node| storage.successors(node).count() == 3));
    }
}
