use magnitude::Magnitude;
use std::any::Any;

use crate::graph::EdgeType;
use crate::provide;
use crate::storage::{GraphStorage, Storage};

pub struct SimpleGraph<W> {
    storage: Box<dyn GraphStorage<W>>,
}

impl<W: Any + Copy> SimpleGraph<W> {
    pub fn init(storage: Storage, edge_type: EdgeType) -> Self {
        SimpleGraph {
            storage: storage.init::<W>(edge_type),
        }
    }

    pub fn init_with_storage(storage: Box<dyn GraphStorage<W>>) -> Self {
        SimpleGraph { storage }
    }
}

impl<W> provide::Neighbors for SimpleGraph<W> {
    fn neighbors(&self, v_index: usize) -> Vec<usize> {
        self.storage.neighbors(v_index)
    }
}

impl<W> provide::Vertices for SimpleGraph<W> {
    fn vertices(&self) -> Vec<usize> {
        self.storage.vertices()
    }

    fn vertex_count(&self) -> usize {
        self.storage.vertex_count()
    }
}

impl<W> provide::Edges<W> for SimpleGraph<W> {
    fn edges(&self) -> Vec<(usize, usize, Magnitude<W>)> {
        self.storage.edges()
    }

    fn edges_from(&self, src_index: usize) -> Vec<(usize, Magnitude<W>)> {
        self.storage.edges_from(src_index)
    }
}

impl<W> provide::Graph<W> for SimpleGraph<W> {
    fn add_vertex(&mut self) -> usize {
        self.storage.add_vertex()
    }

    fn remove_vertex(&mut self, v_id: usize) {
        self.storage.remove_vertex(v_id);
    }

    fn add_edge(&mut self, v1: usize, v2: usize, weight: Magnitude<W>) {
        if v1 == v2 {
            panic!("Can not create loop in simple graph")
        }

        self.storage.add_edge(v1, v2, weight);
    }

    fn remove_edge(&mut self, v1: usize, v2: usize) -> Magnitude<W> {
        self.storage.remove_edge(v1, v2)
    }

    fn is_directed(&self) -> bool {
        self.storage.is_directed()
    }
}
