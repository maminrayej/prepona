mod id_map;

pub use id_map::IdMap;

use crate::graph::{Edge, EdgeType};

pub trait Neighbors {
    fn neighbors(&self, src_id: usize) -> Vec<usize>;
}

pub trait Vertices {
    fn vertices(&self) -> Vec<usize>;

    fn vertex_count(&self) -> usize {
        self.vertices().len()
    }

    fn continuos_id_map(&self) -> IdMap {
        let mut id_map = IdMap::init();

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
    fn edge(&self, src_id: usize, dst_id: usize) -> Option<&E>;

    fn has_edge(&self, src_id: usize, dst_id: usize) -> bool;

    fn edges(&self) -> Vec<(usize, usize, &E)>;

    fn edges_from(&self, src_id: usize) -> Vec<(usize, &E)> {
        // 1. From triplets produced by `edges` function, only keep those that their source vertex id is `src_id`.
        // 2. Map each triplet to a pair by discarding the source vertex id
        self.edges()
            .into_iter()
            .filter(|(s_id, _, _)| *s_id == src_id)
            .map(|(_, dst_id, edge)| (dst_id, edge))
            .collect()
    }

    fn edges_count(&self) -> usize {
        self.edges().len()
    }
}

pub trait Direction {
    fn is_directed() -> bool;

    fn is_undirected() -> bool;
}

pub trait Graph<W, E: Edge<W>, Ty: EdgeType> {
    fn add_vertex(&mut self) -> usize;

    fn remove_vertex(&mut self, vertex_id: usize);

    fn add_edge(&mut self, src_id: usize, dst_id: usize, edge: E);

    fn update_edge(&mut self, src_id: usize, dst_id: usize, edge: E);

    fn remove_edge(&mut self, src_id: usize, dst_id: usize) -> E;
}
