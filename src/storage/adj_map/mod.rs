mod iters;

pub use iters::*;

use std::collections::HashSet;
use std::marker::PhantomData;

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

    type Neighbors<'a> = Neighbors<'a> where Dir: 'a;

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

    fn neighbors(&self, node: NodeId) -> Self::Neighbors<'_> {
        Neighbors {
            iter: self.nodes[&node].iter(),
            seen: HashSet::new(),
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
        self.edges.contains(&self.sort_nodes(predecessor, node))
    }
}

impl<Dir: Direction> AddNodeProvider for AdjMap<Dir> {
    fn add_node(&mut self, node: NodeId) {
        self.nodes.entry(node).or_insert_with(Vec::new);
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
        let sorted_nodes = self.sort_nodes(src_node, dst_node);

        if self.edges.insert(sorted_nodes) {
            let (src_node, dst_node) = sorted_nodes;

            self[src_node].push((dst_node, Orientation::Outgoing));

            if src_node != dst_node {
                self[dst_node].push((src_node, Orientation::Incoming));
            }
        }
    }
}

impl<Dir: Direction> DelEdgeProvider for AdjMap<Dir> {
    fn del_edge(&mut self, src_node: NodeId, dst_node: NodeId) {
        let (src_node, dst_node) = self.sort_nodes(src_node, dst_node);

        self.edges.remove(&(src_node, dst_node));

        self[src_node].retain(|(neighbor, _)| *neighbor != dst_node);

        if src_node != dst_node {
            self[dst_node].retain(|(neighbor, _)| *neighbor != src_node);
        }
    }
}

#[cfg(test)]
mod arbitrary {
    use itertools::Itertools;
    use quickcheck::Arbitrary;
    use rand::{thread_rng, Rng};

    use crate::provide::{
        AddEdgeProvider, AddNodeProvider, Direction, EdgeProvider, EmptyStorage, NodeProvider,
    };

    use super::AdjMap;

    impl<Dir: Direction + Arbitrary> Arbitrary for AdjMap<Dir> {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let node_count = usize::arbitrary(g) % 20;

            let mut rng = thread_rng();
            let edge_probability = rng.gen::<f64>() * rng.gen::<f64>();

            let mut adj_map = Self::init();

            let nodes = (0..node_count)
                .map(|node_id| {
                    adj_map.add_node(node_id.into());
                    node_id.into()
                })
                .collect_vec();

            nodes
                .iter()
                .cartesian_product(nodes.iter())
                .for_each(|(src_node, dst_node)| {
                    let p = rng.gen::<f64>();

                    if p <= edge_probability {
                        adj_map.add_edge(*src_node, *dst_node);
                    }
                });

            adj_map
        }

        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            let mut even_graph = Self::init();
            let mut odd_graph = Self::init();

            for (index, node) in self.nodes().enumerate() {
                if index % 2 == 0 {
                    even_graph.add_node(node)
                } else {
                    odd_graph.add_node(node)
                };
            }

            for (src_node, dst_node) in self.edges() {
                if even_graph.contains_node(src_node) && even_graph.contains_node(dst_node) {
                    even_graph.add_edge(src_node, dst_node);
                } else if odd_graph.contains_node(src_node) && odd_graph.contains_node(dst_node) {
                    odd_graph.add_edge(src_node, dst_node);
                }
            }

            let before_count = self.node_count();

            Box::new(
                [even_graph, odd_graph]
                    .into_iter()
                    .filter(move |adj_map| adj_map.node_count() < before_count),
            )
        }
    }
}
