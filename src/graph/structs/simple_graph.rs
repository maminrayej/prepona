use magnitude::Magnitude;
use std::any::Any;

use crate::graph::EdgeType;
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

    pub fn add_vertex(&mut self) -> usize {
        self.storage.add_vertex()
    }

    pub fn remove_vertex(&mut self, v_id: usize) {
        self.storage.remove_vertex(v_id);
    }

    pub fn add_edge(&mut self, v1: usize, v2: usize, weight: Magnitude<W>) {
        if v1 == v2 {
            panic!("Can not create loop in simple graph")
        }

        self.storage.add_edge(v1, v2, weight);
    }

    pub fn remove_edge(&mut self, v1: usize, v2: usize) -> Magnitude<W> {
        self.storage.remove_edge(v1, v2)
    }

    pub fn vertices(&self) -> Vec<usize> {
        self.storage.vertices()
    }

    pub fn vertices_count(&self) -> usize {
        self.storage.vertex_count()
    }

    pub fn edges(&self) -> Vec<(usize, usize, Magnitude<W>)> {
        self.storage.edges()
    }

    pub fn neighbors(&self, src_index: usize) -> Vec<usize> {
        self.storage.neighbors(src_index)
    }
}
