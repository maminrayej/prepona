use crate::gen::Generator;
use crate::provide::{AddEdgeProvider, AddNodeProvider, EmptyStorage, NodeId, Undirected};

use super::{CompleteGraph, PathGraph};

#[derive(Debug, Clone, Copy)]
pub struct LollipopGraph {
    pub complete_graph_size: usize,
    pub path_graph_size: usize,
}

impl LollipopGraph {
    pub fn init(complete_graph_size: usize, path_graph_size: usize) -> LollipopGraph {
        if complete_graph_size < 3 {
            panic!(
                "Cannot form a lollipop graph with less than 3 nodes for its complete graph component: {complete_graph_size} < 3"
            )
        }
        if path_graph_size < 1 {
            panic!("Cannot form a lollipop graph with less than 1 node for its path garph component: {path_graph_size} < 1")
        }

        LollipopGraph {
            complete_graph_size,
            path_graph_size,
        }
    }
}

impl<S> Generator<S> for LollipopGraph
where
    S: EmptyStorage<Dir = Undirected> + AddNodeProvider + AddEdgeProvider,
{
    fn generate_into(&self, storage: &mut S, start_node: NodeId) -> NodeId {
        let path_start_node =
            CompleteGraph::init(self.complete_graph_size).generate_into(storage, start_node);
        let next_node =
            PathGraph::init(self.path_graph_size).generate_into(storage, path_start_node);

        storage.add_edge(start_node, path_start_node);

        next_node
    }
}

#[cfg(test)]
mod arbitrary {
    use quickcheck::Arbitrary;

    use super::LollipopGraph;

    impl Arbitrary for LollipopGraph {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            LollipopGraph {
                complete_graph_size: usize::arbitrary(g) % 32 + 3,
                path_graph_size: usize::arbitrary(g) % 32 + 1,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;

    use crate::gen::Generator;
    use crate::provide::{NodeProvider, Undirected};
    use crate::storage::AdjMap;

    use super::LollipopGraph;

    #[test]
    #[should_panic(
        expected = "Cannot form a lollipop graph with less than 1 node for its path garph component: 0 < 1"
    )]
    fn lollipop_graph_with_path_of_zero() {
        let _: AdjMap<Undirected> = LollipopGraph::init(3, 0).generate();
    }

    #[test]
    #[should_panic(
        expected = "Cannot form a lollipop graph with less than 3 nodes for its complete graph component: 0 < 3"
    )]
    fn lollipop_graph_with_complete_graph_of_zero() {
        let _: AdjMap<Undirected> = LollipopGraph::init(0, 1).generate();
    }

    #[test]
    #[should_panic(
        expected = "Cannot form a lollipop graph with less than 3 nodes for its complete graph component: 1 < 3"
    )]
    fn lollipop_graph_with_complete_graph_of_one() {
        let _: AdjMap<Undirected> = LollipopGraph::init(1, 1).generate();
    }

    #[test]
    #[should_panic(
        expected = "Cannot form a lollipop graph with less than 3 nodes for its complete graph component: 2 < 3"
    )]
    fn lollipop_graph_with_complete_graph_of_two() {
        let _: AdjMap<Undirected> = LollipopGraph::init(2, 1).generate();
    }

    #[quickcheck]
    fn lollipop_graph_undirected(generator: LollipopGraph) {
        let graph: AdjMap<Undirected> = generator.generate();

        if generator.complete_graph_size == 3 {
            assert_eq!(
                graph
                    .nodes()
                    .filter(|node| graph.successors(*node).count() == 2)
                    .count(),
                generator.complete_graph_size - 1 + generator.path_graph_size - 1
            );
            assert_eq!(
                graph
                    .nodes()
                    .filter(|node| graph.successors(*node).count() == generator.complete_graph_size)
                    .count(),
                1
            );
            assert_eq!(
                graph
                    .nodes()
                    .filter(|node| graph.successors(*node).count() == 1)
                    .count(),
                1
            );
        } else {
            assert_eq!(
                graph
                    .nodes()
                    .filter(
                        |node| graph.successors(*node).count() == generator.complete_graph_size - 1
                    )
                    .count(),
                generator.complete_graph_size - 1
            );
            assert_eq!(
                graph
                    .nodes()
                    .filter(|node| graph.successors(*node).count() == generator.complete_graph_size)
                    .count(),
                1
            );
            assert_eq!(
                graph
                    .nodes()
                    .filter(|node| graph.successors(*node).count() == 2)
                    .count(),
                generator.path_graph_size - 1
            );
            assert_eq!(
                graph
                    .nodes()
                    .filter(|node| graph.successors(*node).count() == 1)
                    .count(),
                1
            );
        }
    }
}
