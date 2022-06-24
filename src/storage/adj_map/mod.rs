mod iters;

pub use iters::*;

use std::marker::PhantomData;

use indexmap::map::Entry::{Occupied, Vacant};
use indexmap::{IndexMap, IndexSet};

use crate::provide::{
    AddEdgeProvider, AddNodeProvider, DelEdgeProvider, DelNodeProvider, Direction, EdgeProvider,
    EmptyStorage, NodeId, NodeProvider, Orientation, Storage,
};

#[derive(Debug, Clone)]
pub struct AdjMap<Dir> {
    nodes: IndexMap<NodeId, Vec<(NodeId, Orientation)>>,
    edges: IndexSet<(NodeId, NodeId)>,

    phantom_dir: PhantomData<Dir>,
}

impl<Dir: Direction> AdjMap<Dir> {
    fn sort_nodes(&self, node1: NodeId, node2: NodeId) -> (NodeId, NodeId) {
        if Dir::is_directed() || node1 <= node2 {
            (node1, node2)
        } else {
            (node2, node1)
        }
    }

    fn orient_nodes(
        &self,
        node1: NodeId,
        node2: NodeId,
        orientation: Orientation,
    ) -> (NodeId, NodeId) {
        if orientation == Orientation::Outgoing {
            (node1, node2)
        } else {
            (node2, node1)
        }
    }
}

mod index_impl {
    use std::ops::{Index, IndexMut};

    use crate::provide::{NodeId, Orientation};

    use super::AdjMap;

    impl<Dir> Index<NodeId> for AdjMap<Dir> {
        type Output = Vec<(NodeId, Orientation)>;

        fn index(&self, index: NodeId) -> &Self::Output {
            &self.nodes[&index]
        }
    }

    impl<Dir> IndexMut<NodeId> for AdjMap<Dir> {
        fn index_mut(&mut self, index: NodeId) -> &mut Self::Output {
            self.nodes.get_mut(&index).unwrap()
        }
    }
}

impl<Dir: Direction> Storage for AdjMap<Dir> {
    type Dir = Dir;
}

impl<Dir: Direction> EmptyStorage for AdjMap<Dir> {
    fn init() -> Self {
        AdjMap {
            nodes: IndexMap::new(),
            edges: IndexSet::new(),
            phantom_dir: PhantomData,
        }
    }
}

impl<Dir: Direction> NodeProvider for AdjMap<Dir> {
    type Nodes<'a> = Nodes<'a> where Dir: 'a;

    type Successors<'a> = OrientedNeighbors<'a, Dir> where Dir: 'a;

    type Predecessors<'a> = OrientedNeighbors<'a, Dir> where Dir: 'a;

    fn contains_node(&self, node: NodeId) -> bool {
        self.nodes.contains_key(&node)
    }

    fn node_count(&self) -> usize {
        self.nodes.len()
    }

    fn nodes(&self) -> Self::Nodes<'_> {
        Nodes {
            iter: self.nodes.keys(),
        }
    }

    fn successors(&self, node: NodeId) -> Self::Successors<'_> {
        OrientedNeighbors {
            iter: self.nodes[&node].iter(),
            orientation: Orientation::Outgoing,
            phantom_dir: PhantomData,
        }
    }

    fn is_successor(&self, node: NodeId, successor: NodeId) -> bool {
        // FIXME: Can you panic naturally?
        self.edges.contains(&self.sort_nodes(node, successor))
    }

    fn predecessors(&self, node: NodeId) -> Self::Predecessors<'_> {
        OrientedNeighbors {
            iter: self.nodes[&node].iter(),
            orientation: Orientation::Incoming,
            phantom_dir: PhantomData,
        }
    }

    fn is_predecessor(&self, node: NodeId, predecessor: NodeId) -> bool {
        // FIXME: Can you panic naturally?
        self.edges.contains(&self.sort_nodes(predecessor, node))
    }
}

impl<Dir: Direction> AddNodeProvider for AdjMap<Dir> {
    fn add_node(&mut self, node: NodeId) {
        match self.nodes.entry(node) {
            Occupied(_) => panic!("Node with the provided id already exists: {:?}", node),
            Vacant(entry) => entry.insert(Vec::new()),
        };
    }
}

impl<Dir: Direction> DelNodeProvider for AdjMap<Dir> {
    fn del_node(&mut self, node: NodeId) {
        let links = self.nodes.swap_remove(&node).unwrap();

        for (neighbor, orientation) in links {
            if neighbor != node {
                self[neighbor].retain(|(n, _)| *n != node);
            }

            let edge_id = if Dir::is_directed() {
                self.orient_nodes(node, neighbor, orientation)
            } else {
                self.sort_nodes(node, neighbor)
            };

            self.edges.swap_remove(&edge_id);
        }
    }
}

impl<Dir: Direction> EdgeProvider for AdjMap<Dir> {
    type Edges<'a> = Edges<'a> where Dir: 'a;

    type IncomingEdges<'a> = OrientedNeighbors<'a, Dir> where Dir: 'a;

    type OutgoingEdges<'a> = OrientedNeighbors<'a, Dir> where Dir: 'a;

    fn contains_edge(&self, src_node: NodeId, dst_node: NodeId) -> bool {
        // FIXME: Can you panic naturally?
        self.edges
            .get(&self.sort_nodes(src_node, dst_node))
            .is_some()
    }

    fn edge_count(&self) -> usize {
        self.edges.len()
    }

    fn edges(&self) -> Self::Edges<'_> {
        Edges {
            iter: self.edges.iter(),
        }
    }

    fn incoming_edges(&self, node: NodeId) -> Self::IncomingEdges<'_> {
        OrientedNeighbors {
            iter: self.nodes[&node].iter(),
            orientation: Orientation::Incoming,
            phantom_dir: PhantomData,
        }
    }

    fn outgoing_edges(&self, node: NodeId) -> Self::OutgoingEdges<'_> {
        OrientedNeighbors {
            iter: self.nodes[&node].iter(),
            orientation: Orientation::Outgoing,
            phantom_dir: PhantomData,
        }
    }

    fn in_degree(&self, node: NodeId) -> usize {
        self.incoming_edges(node).count()
    }

    fn out_degree(&self, node: NodeId) -> usize {
        self.outgoing_edges(node).count()
    }
}

impl<Dir: Direction> AddEdgeProvider for AdjMap<Dir> {
    fn add_edge(&mut self, src_node: NodeId, dst_node: NodeId) {
        // FIXME: does not panic if nodes are invalid
        let sorted_nodes = self.sort_nodes(src_node, dst_node);

        self.edges
            .insert(sorted_nodes)
            .then(|| {
                let (src_node, dst_node) = sorted_nodes;

                self[src_node].push((dst_node, Orientation::Outgoing));

                if src_node != dst_node {
                    self[dst_node].push((src_node, Orientation::Incoming));
                }
            })
            .unwrap();
    }
}

impl<Dir: Direction> DelEdgeProvider for AdjMap<Dir> {
    fn del_edge(&mut self, src_node: NodeId, dst_node: NodeId) {
        let (src_node, dst_node) = self.sort_nodes(src_node, dst_node);

        self.edges
            .remove(&(src_node, dst_node))
            .then(|| {
                self[src_node].retain(|(neighbor, _)| *neighbor != dst_node);

                if src_node != dst_node {
                    self[dst_node].retain(|(neighbor, _)| *neighbor != src_node);
                }
            })
            .unwrap();
    }
}

crate::provide::test_util::impl_arbitrary!(AdjMap);

crate::provide::test_util::impl_test_suite!(AdjMap);

#[cfg(test)]
mod tests {
    use rand::prelude::IteratorRandom;

    use crate::{
        provide::{
            AddEdgeProvider, AddNodeProvider, DelEdgeProvider, DelNodeProvider, Directed,
            Direction, EdgeProvider, NodeId, NodeProvider, Undirected,
        },
        storage::AdjMap,
    };

    fn new_node_id<P>(provider: &P) -> NodeId
    where
        P: NodeProvider,
    {
        provider.nodes().max().map_or(0.into(), |node| node + 1)
    }

    #[should_panic]
    #[test]
    fn adj_map_successors_of_invalid_node() {
        fn test<Dir: Direction>(adj_map: AdjMap<Dir>) -> bool {
            let invalid_node = new_node_id(&adj_map);

            adj_map.successors(invalid_node);

            true
        }

        quickcheck::quickcheck(test as fn(AdjMap<Directed>) -> bool);
        quickcheck::quickcheck(test as fn(AdjMap<Undirected>) -> bool);
    }

    #[should_panic]
    #[test]
    fn adj_map_predecessors_of_invalid_node() {
        fn test<Dir: Direction>(adj_map: AdjMap<Dir>) -> bool {
            let invalid_node = new_node_id(&adj_map);

            adj_map.predecessors(invalid_node);

            true
        }

        quickcheck::quickcheck(test as fn(AdjMap<Directed>) -> bool);
        quickcheck::quickcheck(test as fn(AdjMap<Undirected>) -> bool);
    }

    #[should_panic]
    #[test]
    fn adj_map_add_duplicate_node() {
        fn test<Dir: Direction>(mut adj_map: AdjMap<Dir>) -> bool {
            let existing_node = adj_map.nodes().choose(&mut rand::thread_rng()).unwrap();

            adj_map.add_node(existing_node);

            true
        }

        quickcheck::quickcheck(test as fn(AdjMap<Directed>) -> bool);
        quickcheck::quickcheck(test as fn(AdjMap<Undirected>) -> bool);
    }

    #[should_panic]
    #[test]
    fn adj_map_delete_non_existing_node() {
        fn test<Dir: Direction>(mut adj_map: AdjMap<Dir>) -> bool {
            let invalid_node = new_node_id(&adj_map);

            adj_map.del_node(invalid_node);

            true
        }

        quickcheck::quickcheck(test as fn(AdjMap<Directed>) -> bool);
        quickcheck::quickcheck(test as fn(AdjMap<Undirected>) -> bool);
    }

    #[should_panic]
    #[test]
    fn adj_map_incoming_edges_of_invalid_node() {
        fn test<Dir: Direction>(adj_map: AdjMap<Dir>) -> bool {
            let invalid_node = new_node_id(&adj_map);

            adj_map.incoming_edges(invalid_node);

            true
        }

        quickcheck::quickcheck(test as fn(AdjMap<Directed>) -> bool);
        quickcheck::quickcheck(test as fn(AdjMap<Undirected>) -> bool);
    }

    #[should_panic]
    #[test]
    fn adj_map_outgoing_edges_of_invalid_node() {
        fn test<Dir: Direction>(adj_map: AdjMap<Dir>) -> bool {
            let invalid_node = new_node_id(&adj_map);

            adj_map.outgoing_edges(invalid_node);

            true
        }

        quickcheck::quickcheck(test as fn(AdjMap<Directed>) -> bool);
        quickcheck::quickcheck(test as fn(AdjMap<Undirected>) -> bool);
    }

    #[should_panic]
    #[test]
    fn adj_map_in_degree_of_invalid_node() {
        fn test<Dir: Direction>(adj_map: AdjMap<Dir>) -> bool {
            let invalid_node = new_node_id(&adj_map);

            adj_map.in_degree(invalid_node);

            true
        }

        quickcheck::quickcheck(test as fn(AdjMap<Directed>) -> bool);
        quickcheck::quickcheck(test as fn(AdjMap<Undirected>) -> bool);
    }

    #[should_panic]
    #[test]
    fn adj_map_out_degree_of_invalid_node() {
        fn test<Dir: Direction>(adj_map: AdjMap<Dir>) -> bool {
            let invalid_node = new_node_id(&adj_map);

            adj_map.out_degree(invalid_node);

            true
        }

        quickcheck::quickcheck(test as fn(AdjMap<Directed>) -> bool);
        quickcheck::quickcheck(test as fn(AdjMap<Undirected>) -> bool);
    }

    #[should_panic]
    #[test]
    fn adj_map_add_multi_edge() {
        fn test<Dir: Direction>(mut adj_map: AdjMap<Dir>) -> bool {
            let (src_node, dst_node) = adj_map.edges().choose(&mut rand::thread_rng()).unwrap();

            adj_map.add_edge(src_node, dst_node);

            true
        }

        quickcheck::quickcheck(test as fn(AdjMap<Directed>) -> bool);
        quickcheck::quickcheck(test as fn(AdjMap<Undirected>) -> bool);
    }

    #[should_panic]
    #[test]
    fn adj_map_del_non_existing_edge() {
        fn test<Dir: Direction>(mut adj_map: AdjMap<Dir>) -> bool {
            let new_node = new_node_id(&adj_map);
            let valid_node = adj_map.nodes().choose(&mut rand::thread_rng()).unwrap();

            adj_map.add_node(new_node);

            adj_map.del_edge(new_node, valid_node);

            true
        }

        quickcheck::quickcheck(test as fn(AdjMap<Directed>) -> bool);
        quickcheck::quickcheck(test as fn(AdjMap<Undirected>) -> bool);
    }

    #[should_panic]
    #[test]
    fn adj_map_del_edge_with_invalid_src() {
        fn test<Dir: Direction>(mut adj_map: AdjMap<Dir>) -> bool {
            let invalid_node = new_node_id(&adj_map);
            let valid_node = adj_map.nodes().choose(&mut rand::thread_rng()).unwrap();

            adj_map.del_edge(invalid_node, valid_node);

            true
        }

        quickcheck::quickcheck(test as fn(AdjMap<Directed>) -> bool);
        quickcheck::quickcheck(test as fn(AdjMap<Undirected>) -> bool);
    }

    #[should_panic]
    #[test]
    fn adj_map_del_edge_with_invalid_dst() {
        fn test<Dir: Direction>(mut adj_map: AdjMap<Dir>) -> bool {
            let invalid_node = new_node_id(&adj_map);
            let valid_node = adj_map.nodes().choose(&mut rand::thread_rng()).unwrap();

            adj_map.del_edge(valid_node, invalid_node);

            true
        }

        quickcheck::quickcheck(test as fn(AdjMap<Directed>) -> bool);
        quickcheck::quickcheck(test as fn(AdjMap<Undirected>) -> bool);
    }
}
