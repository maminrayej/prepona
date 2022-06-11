use crate::gen::Generator;
use crate::provide::{AddEdgeProvider, AddNodeProvider, EmptyStorage, NodeId, Undirected};

use super::CycleGraph;

#[derive(Debug, Clone, Copy)]
pub struct CircularLadderGraph {
    component_size: usize,
}

impl CircularLadderGraph {
    pub fn init(component_size: usize) -> CircularLadderGraph {
        if component_size < 3 {
            panic!("Can not generate a circular ladder graph graph with a component size smaller than 3: ${component_size} < 3")
        }

        CircularLadderGraph { component_size }
    }
}

impl<S> Generator<S> for CircularLadderGraph
where
    S: EmptyStorage<Dir = Undirected> + AddNodeProvider + AddEdgeProvider,
{
    fn generate_into(&self, storage: &mut S, start_node: NodeId) -> NodeId {
        let start_of_cycle2 =
            CycleGraph::init(self.component_size).generate_into(storage, start_node);
        let next_node =
            CycleGraph::init(self.component_size).generate_into(storage, start_of_cycle2);

        let zip = start_node
            .range(self.component_size)
            .zip(start_of_cycle2.range(self.component_size));

        for (src_node, dst_node) in zip {
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
                component_size: usize::arbitrary(g) % 32 + 3,
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

    #[quickcheck]
    fn circular_ladder_graph_undirected(generator: CircularLadderGraph) {
        let storage: AdjMap<Undirected> = generator.generate();

        assert_eq!(storage.node_count(), generator.component_size * 2);
        assert_eq!(storage.edge_count(), generator.component_size * 3);

        assert!(storage
            .nodes()
            .all(|node| storage.successors(node).count() == 3));
    }
}
