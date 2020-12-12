mod id_map;

pub use id_map::IdMap;

use crate::graph::{Edge, EdgeDir};

pub trait Neighbors {
    fn neighbors(&self, src_id: usize) -> Vec<usize>;
}

pub trait Vertices {
    fn vertices(&self) -> Vec<usize>;

    fn vertex_count(&self) -> usize {
        self.vertices().len()
    }

    fn continuos_id_map(&self) -> IdMap {
        let vertex_count = self.vertex_count();

        let mut id_map = IdMap::init(vertex_count);

        self.vertices()
            .iter()
            .enumerate()
            .for_each(|(virt_id, &real_id)| {
                id_map.put_virt_to_real(virt_id, real_id);
                id_map.put_real_to_virt(real_id, virt_id);
            });

        id_map
    }
}

pub trait Edges<W, E: Edge<W>> {
    fn edges_from(&self, src_id: usize) -> Vec<(usize, &E)>;

    fn edges_between(&self, src_id: usize, dst_id: usize) -> Vec<&E>;

    fn edge_between(&self, src_id: usize, dst_id: usize, edge_id: usize) -> Option<&E>;

    fn edge(&self, edge_id: usize) -> Option<&E>;

    fn has_any_edge(&self, src_id: usize, dst_id: usize) -> bool;

    fn edges(&self) -> Vec<(usize, usize, &E)>;

    fn as_directed_edges(&self) -> Vec<(usize, usize, &E)>;

    fn edges_count(&self) -> usize;
}

pub trait Graph<W, E: Edge<W>, Ty: EdgeDir> {
    fn add_vertex(&mut self) -> usize;

    fn remove_vertex(&mut self, vertex_id: usize);

    fn add_edge(&mut self, src_id: usize, dst_id: usize, edge: E) -> usize;

    fn update_edge(&mut self, src_id: usize, dst_id: usize, edge_id: usize, edge: E);

    fn remove_edge(&mut self, src_id: usize, dst_id: usize, edge_id: usize) -> Option<E>;
}
