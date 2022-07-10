use std::iter::Copied;
use std::marker::PhantomData;

use indexmap::IndexSet;
use itertools::Itertools;

use crate::provide::{Direction, EdgeProvider, NodeId, NodeProvider, Storage};

use super::FrozenView;

pub struct GenericView<'i, G> {
    inner: &'i G,
    nodes: IndexSet<NodeId>,
    edges: IndexSet<(NodeId, NodeId)>,
}

impl<'i, G> GenericView<'i, G> {
    pub fn new(
        inner: &'i G,
        nodes: impl Iterator<Item = NodeId>,
        edges: impl Iterator<Item = (NodeId, NodeId)>,
    ) -> Self {
        Self {
            inner,
            nodes: nodes.collect(),
            edges: edges.collect(),
        }
    }
}

pub struct Successors<'a, Dir: Direction> {
    target_node: NodeId,
    inner: indexmap::set::Iter<'a, (NodeId, NodeId)>,
    phantom_dir: PhantomData<Dir>,
}

impl<'a, Dir: Direction> Successors<'a, Dir> {
    pub fn new(target_node: NodeId, inner: indexmap::set::Iter<'a, (NodeId, NodeId)>) -> Self {
        Self {
            target_node,
            inner,
            phantom_dir: PhantomData,
        }
    }
}

impl<'a, Dir: Direction> Iterator for Successors<'a, Dir> {
    type Item = NodeId;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.find_map(|(src_node, dst_node)| {
            if *src_node == self.target_node {
                Some(*dst_node)
            } else if Dir::is_undirected() && *dst_node == self.target_node {
                Some(*src_node)
            } else {
                None
            }
        })
    }
}

pub struct Predecessors<'a, Dir: Direction> {
    target_node: NodeId,
    inner: indexmap::set::Iter<'a, (NodeId, NodeId)>,
    phantom_dir: PhantomData<Dir>,
}

impl<'a, Dir: Direction> Predecessors<'a, Dir> {
    pub fn new(target_node: NodeId, inner: indexmap::set::Iter<'a, (NodeId, NodeId)>) -> Self {
        Self {
            target_node,
            inner,
            phantom_dir: PhantomData,
        }
    }
}

impl<'a, Dir: Direction> Iterator for Predecessors<'a, Dir> {
    type Item = NodeId;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.find_map(|(src_node, dst_node)| {
            if *dst_node == self.target_node {
                Some(*src_node)
            } else if Dir::is_undirected() && *src_node == self.target_node {
                Some(*dst_node)
            } else {
                None
            }
        })
    }
}
impl<'i, G> Storage for GenericView<'i, G>
where
    G: Storage,
{
    type Dir = G::Dir;
}

impl<'i, G> NodeProvider for GenericView<'i, G>
where
    G: NodeProvider,
{
    type Nodes<'a> = Copied<indexmap::set::Iter<'a, NodeId>> where Self: 'a;

    type Successors<'a> = Successors<'a, G::Dir> where Self: 'a;

    type Predecessors<'a> = Predecessors<'a, G::Dir> where Self: 'a;

    fn contains_node(&self, node: NodeId) -> bool {
        self.nodes.contains(&node)
    }

    fn node_count(&self) -> usize {
        self.nodes.len()
    }

    fn nodes(&self) -> Self::Nodes<'_> {
        self.nodes.iter().copied()
    }

    fn successors(&self, node: NodeId) -> Self::Successors<'_> {
        Successors::new(node, self.edges.iter())
    }

    fn predecessors(&self, node: NodeId) -> Self::Predecessors<'_> {
        Predecessors::new(node, self.edges.iter())
    }

    fn is_successor(&self, node: NodeId, successor: NodeId) -> bool {
        self.successors(node).contains(&successor)
    }

    fn is_predecessor(&self, node: NodeId, predecessor: NodeId) -> bool {
        self.predecessors(node).contains(&predecessor)
    }
}

impl<'i, G> EdgeProvider for GenericView<'i, G>
where
    G: EdgeProvider,
{
    type Edges<'a> = Copied<indexmap::set::Iter<'a, (NodeId, NodeId)>> where Self: 'a;

    type IncomingEdges<'a> = Self::Predecessors<'a> where Self: 'a;

    type OutgoingEdges<'a> = Self::Successors<'a> where Self: 'a;

    fn contains_edge(&self, src_node: NodeId, dst_node: NodeId) -> bool {
        self.edges.contains(&(src_node, dst_node))
            || (G::Dir::is_undirected() && self.edges.contains(&(dst_node, src_node)))
    }

    fn edge_count(&self) -> usize {
        self.edges.len()
    }

    fn edges(&self) -> Self::Edges<'_> {
        self.edges.iter().copied()
    }

    fn incoming_edges(&self, node: NodeId) -> Self::IncomingEdges<'_> {
        self.predecessors(node)
    }

    fn outgoing_edges(&self, node: NodeId) -> Self::OutgoingEdges<'_> {
        self.successors(node)
    }

    fn in_degree(&self, node: NodeId) -> usize {
        self.incoming_edges(node).count()
    }

    fn out_degree(&self, node: NodeId) -> usize {
        self.outgoing_edges(node).count()
    }
}

impl<'i, G> FrozenView for GenericView<'i, G>
where
    G: NodeProvider + EdgeProvider,
{
    type Graph = G;

    fn inner(&self) -> &Self::Graph {
        self.inner
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use quickcheck_macros::quickcheck;

    use crate::gen::{CompleteGraph, CycleGraph, Generator, PathGraph};
    use crate::provide::{Directed, EdgeProvider, NodeProvider, Undirected};
    use crate::storage::AdjMap;

    use super::GenericView;

    #[quickcheck]
    fn generic_view_of_undirected_complete_graph(generator: CompleteGraph) {
        let graph: AdjMap<Undirected> = generator.generate();

        let view = GenericView::new(
            &graph,
            graph.nodes().filter(|node| node.inner() % 2 == 0),
            graph.edges().filter(|(src_node, dst_node)| {
                src_node.inner() % 2 == 0 && dst_node.inner() % 2 == 0
            }),
        );

        let expected_node_count = (graph.node_count() / 2) + graph.node_count() % 2;

        assert_eq!(view.node_count(), expected_node_count);
        assert_eq!(
            view.edge_count(),
            expected_node_count * (expected_node_count - 1) / 2
        );
        assert!(view
            .nodes()
            .all(|node| view.successors(node).count() == expected_node_count - 1));

        assert!(view
            .nodes()
            .all(|node| view.predecessors(node).count() == expected_node_count - 1));

        assert!(view
            .nodes()
            .all(|node| view.incoming_edges(node).count() == expected_node_count - 1));

        assert!(view
            .nodes()
            .all(|node| view.outgoing_edges(node).count() == expected_node_count - 1));

        assert!(view
            .nodes()
            .all(|node| view.in_degree(node) == expected_node_count - 1));

        assert!(view
            .nodes()
            .all(|node| view.out_degree(node) == expected_node_count - 1));

        for node1 in view.nodes() {
            for node2 in view.nodes() {
                if node1 != node2 {
                    assert!(view.predecessors(node1).contains(&node2));
                    assert!(view.successors(node1).contains(&node2));
                }
            }
        }
    }

    #[quickcheck]
    fn generic_view_of_directed_complete_graph(generator: CompleteGraph) {
        let graph: AdjMap<Directed> = generator.generate();

        let view = GenericView::new(
            &graph,
            graph.nodes().filter(|node| node.inner() % 2 == 0),
            graph.edges().filter(|(src_node, dst_node)| {
                src_node.inner() % 2 == 0 && dst_node.inner() % 2 == 0
            }),
        );

        let expected_node_count = (graph.node_count() / 2) + graph.node_count() % 2;

        assert_eq!(view.node_count(), expected_node_count);
        assert_eq!(
            view.edge_count(),
            expected_node_count * (expected_node_count - 1)
        );
        assert!(view
            .nodes()
            .all(|node| view.successors(node).count() == expected_node_count - 1));

        assert!(view
            .nodes()
            .all(|node| view.predecessors(node).count() == expected_node_count - 1));

        assert!(view
            .nodes()
            .all(|node| view.incoming_edges(node).count() == expected_node_count - 1));

        assert!(view
            .nodes()
            .all(|node| view.outgoing_edges(node).count() == expected_node_count - 1));

        assert!(view
            .nodes()
            .all(|node| view.in_degree(node) == expected_node_count - 1));

        assert!(view
            .nodes()
            .all(|node| view.out_degree(node) == expected_node_count - 1));

        for node1 in view.nodes() {
            for node2 in view.nodes() {
                if node1 != node2 {
                    assert!(view.predecessors(node1).contains(&node2));
                    assert!(view.successors(node1).contains(&node2));
                }
            }
        }
    }

    #[quickcheck]
    fn generic_view_of_undirected_path_graph(generator: PathGraph) {
        let graph: AdjMap<Undirected> = generator.generate();

        let view = GenericView::new(
            &graph,
            graph.nodes().filter(|node| node.inner() % 2 == 0),
            graph.edges().filter(|(src_node, dst_node)| {
                src_node.inner() % 2 == 0 && dst_node.inner() % 2 == 0
            }),
        );

        let expected_node_count = (graph.node_count() / 2) + graph.node_count() % 2;

        assert_eq!(view.node_count(), expected_node_count);
        assert_eq!(view.edge_count(), 0);

        assert!(view.nodes().all(|node| view.successors(node).count() == 0));

        assert!(view
            .nodes()
            .all(|node| view.predecessors(node).count() == 0));

        assert!(view
            .nodes()
            .all(|node| view.incoming_edges(node).count() == 0));

        assert!(view
            .nodes()
            .all(|node| view.outgoing_edges(node).count() == 0));

        assert!(view.nodes().all(|node| view.in_degree(node) == 0));

        assert!(view.nodes().all(|node| view.out_degree(node) == 0));

        for node1 in view.nodes() {
            for node2 in view.nodes() {
                if node1 != node2 {
                    assert!(!view.predecessors(node1).contains(&node2));
                    assert!(!view.successors(node1).contains(&node2));
                }
            }
        }
    }

    #[quickcheck]
    fn generic_view_of_directed_path_graph(generator: PathGraph) {
        let graph: AdjMap<Directed> = generator.generate();

        let view = GenericView::new(
            &graph,
            graph.nodes().filter(|node| node.inner() % 2 == 0),
            graph.edges().filter(|(src_node, dst_node)| {
                src_node.inner() % 2 == 0 && dst_node.inner() % 2 == 0
            }),
        );

        let expected_node_count = (graph.node_count() / 2) + graph.node_count() % 2;

        assert_eq!(view.node_count(), expected_node_count);
        assert_eq!(view.edge_count(), 0);

        assert!(view.nodes().all(|node| view.successors(node).count() == 0));

        assert!(view
            .nodes()
            .all(|node| view.predecessors(node).count() == 0));

        assert!(view
            .nodes()
            .all(|node| view.incoming_edges(node).count() == 0));

        assert!(view
            .nodes()
            .all(|node| view.outgoing_edges(node).count() == 0));

        assert!(view.nodes().all(|node| view.in_degree(node) == 0));

        assert!(view.nodes().all(|node| view.out_degree(node) == 0));

        for node1 in view.nodes() {
            for node2 in view.nodes() {
                if node1 != node2 {
                    assert!(!view.predecessors(node1).contains(&node2));
                    assert!(!view.successors(node1).contains(&node2));
                }
            }
        }
    }

    #[quickcheck]
    fn generic_view_of_undirected_halved_path_graph(generator: PathGraph) {
        let storage: AdjMap<Undirected> = generator.generate();

        let view = GenericView::new(
            &storage,
            storage
                .nodes()
                .filter(|node| node.inner() < storage.node_count() / 2),
            storage.edges().filter(|(node1, node2)| {
                node1.inner() < storage.node_count() / 2 && node2.inner() < storage.node_count() / 2
            }),
        );

        let expected_node_count = storage.node_count() / 2;
        if expected_node_count != 0 {
            assert_eq!(view.node_count(), expected_node_count);

            assert_eq!(view.edge_count(), expected_node_count - 1);

            if expected_node_count > 1 {
                assert_eq!(
                    view.nodes()
                        .filter(|node| view.successors(*node).count() == 1)
                        .count(),
                    2
                );

                assert_eq!(
                    view.nodes()
                        .filter(|node| view.successors(*node).count() == 2)
                        .count(),
                    expected_node_count - 2
                )
            }
        }
    }

    #[quickcheck]
    fn generic_view_of_directed_halved_path_graph(generator: PathGraph) {
        let storage: AdjMap<Directed> = generator.generate();

        let view = GenericView::new(
            &storage,
            storage
                .nodes()
                .filter(|node| node.inner() < storage.node_count() / 2),
            storage.edges().filter(|(node1, node2)| {
                node1.inner() < storage.node_count() / 2 && node2.inner() < storage.node_count() / 2
            }),
        );

        let expected_node_count = storage.node_count() / 2;
        assert_eq!(view.node_count(), expected_node_count);

        if expected_node_count != 0 {
            assert_eq!(view.edge_count(), expected_node_count - 1);

            assert_eq!(
                view.nodes()
                    .filter(|node| view.outgoing_edges(*node).count() == 0)
                    .count(),
                1
            );

            assert_eq!(
                view.nodes()
                    .filter(|node| view.incoming_edges(*node).count() == 0)
                    .count(),
                1
            );

            assert_eq!(
                view.nodes()
                    .filter(|node| view.outgoing_edges(*node).count() == 1)
                    .count(),
                expected_node_count - 1
            );

            assert_eq!(
                view.nodes()
                    .filter(|node| view.incoming_edges(*node).count() == 1)
                    .count(),
                expected_node_count - 1
            );
        }
    }

    #[quickcheck]
    fn generic_view_of_undirected_cycle_graph(generator: CycleGraph) {
        let storage: AdjMap<Undirected> = generator.generate();

        let view = GenericView::new(
            &storage,
            storage.nodes().filter(|node| node.inner() != 0),
            storage
                .edges()
                .filter(|(node1, node2)| node1.inner() != 0 && node2.inner() != 0),
        );

        let expected_node_count = storage.node_count() - 1;

        assert_eq!(view.node_count(), expected_node_count);

        assert_eq!(view.edge_count(), expected_node_count - 1);

        assert_eq!(
            view.nodes()
                .filter(|node| view.successors(*node).count() == 1)
                .count(),
            2
        );

        assert_eq!(
            view.nodes()
                .filter(|node| view.successors(*node).count() == 2)
                .count(),
            expected_node_count - 2
        )
    }

    #[quickcheck]
    fn generic_view_of_directed_cycle_graph(generator: CycleGraph) {
        let storage: AdjMap<Directed> = generator.generate();

        let view = GenericView::new(
            &storage,
            storage.nodes().filter(|node| node.inner() != 0),
            storage
                .edges()
                .filter(|(node1, node2)| node1.inner() != 0 && node2.inner() != 0),
        );

        let expected_node_count = storage.node_count() - 1;

        assert_eq!(view.node_count(), expected_node_count);

        assert_eq!(view.edge_count(), expected_node_count - 1);

        assert_eq!(
            view.nodes()
                .filter(|node| view.outgoing_edges(*node).count() == 0)
                .count(),
            1
        );

        assert_eq!(
            view.nodes()
                .filter(|node| view.incoming_edges(*node).count() == 0)
                .count(),
            1
        );

        assert_eq!(
            view.nodes()
                .filter(|node| view.outgoing_edges(*node).count() == 1)
                .count(),
            expected_node_count - 1
        );

        assert_eq!(
            view.nodes()
                .filter(|node| view.incoming_edges(*node).count() == 1)
                .count(),
            expected_node_count - 1
        );
    }
}
