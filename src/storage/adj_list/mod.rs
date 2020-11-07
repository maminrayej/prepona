use std::collections::HashSet;
use std::marker::PhantomData;

use crate::graph::{DefaultEdge, Edge, FlowEdge};
use crate::storage::GraphStorage;

/// An `AdjList` that stores edges of type `DefaultEdge`.
pub type List<W> = AdjList<W, DefaultEdge<W>>;

/// An `AdjList` that stores edges of type `FlowEdge`.
pub type FlowList<W> = AdjList<W, FlowEdge<W>>;

pub struct AdjList<W, E: Edge<W>> {
    edges_of: Vec<Vec<(usize, E)>>,
    reusable_ids: HashSet<usize>,

    vertex_count: usize,
    is_directed: bool,

    phantom_w: PhantomData<W>,
}

impl<W, E: Edge<W> + Copy> AdjList<W, E> {
    pub fn init(is_directed: bool) -> Self {
        AdjList {
            edges_of: vec![],
            reusable_ids: HashSet::new(),

            vertex_count: 0,
            is_directed,

            phantom_w: PhantomData,
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

impl<W, E: Edge<W> + Copy> GraphStorage<W, E> for AdjList<W, E> {
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
            self.edges_of[src_id].retain(|(dst_id, _)| *dst_id != vertex_id)
        }

        self.vertex_count -= 1;

        self.reusable_ids.insert(vertex_id);
    }

    fn add_edge(&mut self, src_id: usize, dst_id: usize, edge: E) {
        self.validate_id(src_id);
        self.validate_id(dst_id);

        self.edges_of[src_id].push((dst_id, edge));

        if self.is_undirected() {
            self.edges_of[dst_id].push((src_id, edge))
        }
    }

    fn remove_edge(&mut self, src_id: usize, dst_id: usize) -> E {
        self.validate_id(src_id);
        self.validate_id(dst_id);

        if let Some((index, _)) = self.edges_of[src_id]
            .iter()
            .find(|(d_id, _)| *d_id == dst_id)
        {
            let index = *index;

            if self.is_undirected() {
                self.edges_of[dst_id].retain(|(d_id, _)| *d_id != src_id);
            }

            self.edges_of[src_id].remove(index).1
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

    fn edges(&self, doubles: bool) -> Vec<(usize, usize, &E)> {
        self.vertices()
            .into_iter()
            .flat_map(|src_id| {
                self.edges_from(src_id)
                    .into_iter()
                    .map(|(dst_id, edge)| (src_id, dst_id, edge))
                    .collect::<Vec<(usize, usize, &E)>>()
            })
            .filter(|(v1, v2, edge)| {
                edge.get_weight().is_finite() && if self.is_undirected() && !doubles { v1 <= v2 } else { true }
            })
            .collect()
    }

    fn edges_from(&self, src_id: usize) -> Vec<(usize, &E)> {
        self.validate_id(src_id);

        self.edges_of[src_id]
            .iter()
            .map(|(dst_id, edge)| (*dst_id, edge))
            .collect()
    }

    fn neighbors(&self, src_id: usize) -> Vec<usize> {
        self.validate_id(src_id);

        self.edges_of[src_id]
            .iter()
            .map(|(dst_id, _)| *dst_id)
            .collect()
    }

    fn is_directed(&self) -> bool {
        self.is_directed
    }
}
