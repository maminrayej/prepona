use std::collections::HashSet;
use std::marker::PhantomData;

use crate::graph::{DefaultEdge, DirectedEdge, Edge, EdgeType, FlowEdge, UndirectedEdge};
use crate::storage::GraphStorage;

/// An `AdjList` that stores edges of type `DefaultEdge`.
pub type List<W> = AdjList<W, DefaultEdge<W>, UndirectedEdge>;
pub type DiList<W> = AdjList<W, DefaultEdge<W>, DirectedEdge>;

/// An `AdjList` that stores edges of type `FlowEdge`.
pub type FlowList<W> = AdjList<W, FlowEdge<W>, UndirectedEdge>;
pub type DiFlowList<W> = AdjList<W, FlowEdge<W>, DirectedEdge>;

pub struct AdjList<W, E: Edge<W>, Ty: EdgeType> {
    edges_of: Vec<Vec<E>>,
    reusable_ids: HashSet<usize>,

    vertex_count: usize,
    is_directed: bool,

    phantom_w: PhantomData<W>,
    phantom_ty: PhantomData<Ty>,
}

impl<W: Copy, E: Edge<W> + Copy, Ty: EdgeType> AdjList<W, E, Ty> {
    pub fn init() -> Self {
        AdjList {
            edges_of: vec![],
            reusable_ids: HashSet::new(),

            vertex_count: 0,
            is_directed: Ty::is_directed(),

            phantom_w: PhantomData,
            phantom_ty: PhantomData,
        }
    }

    fn next_reusable_id(&mut self) -> Option<usize> {
        if let Some(id) = self.reusable_ids.iter().take(1).next().copied() {
            self.reusable_ids.remove(&id);

            Some(id)
        } else {
            None
        }
    }

    fn validate_id(&self, vertex_id: usize) {
        if self.reusable_ids.contains(&vertex_id) || vertex_id >= self.vertex_count() {
            panic!(format!(
                "Vertex with id: {} is not present in the graph",
                vertex_id
            ))
        }
    }
}

impl<W: Copy, E: Edge<W> + Copy, Ty: EdgeType> GraphStorage<W, E, Ty> for AdjList<W, E, Ty> {
    fn add_vertex(&mut self) -> usize {
        self.vertex_count += 1;

        if let Some(reusable_id) = self.next_reusable_id() {
            reusable_id
        } else {
            self.edges_of.push(vec![]);

            self.edges_of.len() - 1
        }
    }

    fn remove_vertex(&mut self, vertex_id: usize) {
        self.validate_id(vertex_id);

        self.edges_of[vertex_id].clear();

        for src_id in 0..self.vertex_count() {
            self.edges_of[src_id].retain(|edge| edge.get_dst_id() != vertex_id)
        }

        self.vertex_count -= 1;

        self.reusable_ids.insert(vertex_id);
    }

    fn add_edge(&mut self, edge: E) {
        let (src_id, dst_id) = (edge.get_src_id(), edge.get_dst_id());

        self.validate_id(src_id);
        self.validate_id(dst_id);

        self.edges_of[src_id].push(edge);

        if self.is_undirected() {
            self.edges_of[dst_id].push(E::init(dst_id, src_id, *edge.get_weight()));
        }
    }

    fn remove_edge(&mut self, src_id: usize, dst_id: usize) -> E {
        self.validate_id(src_id);
        self.validate_id(dst_id);

        if let Some((index, _)) = self.edges_of[src_id]
            .iter()
            .enumerate()
            .find(|(_, edge)| edge.get_dst_id() == dst_id)
        {
            if self.is_undirected() {
                self.edges_of[dst_id].retain(|edge| edge.get_dst_id() != src_id);
            }

            self.edges_of[src_id].remove(index)
        } else {
            panic!(
                "There is no edge from vertex: {} to vertex: {}",
                src_id, dst_id
            )
        }
    }

    fn vertex_count(&self) -> usize {
        self.vertex_count
    }

    fn vertices(&self) -> Vec<usize> {
        (0..self.edges_of.len())
            .filter(|vertex_id| !self.edges_of[*vertex_id].is_empty())
            .collect()
    }

    fn edge(&self, src_id: usize, dst_id: usize) -> &E {
        self.validate_id(src_id);
        self.validate_id(dst_id);

        self.edges_of[src_id]
            .iter()
            .find(|edge| edge.get_dst_id() == dst_id)
            .unwrap()
    }

    fn edges(&self) -> Vec<&E> {
        self.vertices()
            .into_iter()
            .flat_map(|src_id| self.edges_from(src_id).into_iter())
            .collect()
    }

    fn edges_from(&self, src_id: usize) -> Vec<&E> {
        self.validate_id(src_id);

        self.edges_of[src_id]
            .iter()
            .filter(|edge| edge.get_weight().is_finite())
            .collect()
    }

    fn neighbors(&self, src_id: usize) -> Vec<usize> {
        self.validate_id(src_id);

        self.edges_of[src_id]
            .iter()
            .map(|edge| edge.get_dst_id())
            .collect()
    }

    fn is_directed(&self) -> bool {
        self.is_directed
    }
}
