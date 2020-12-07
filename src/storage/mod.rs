mod adj_list;
mod adj_matrix;

pub use adj_list::{AdjList, DiFlowList, DiList, FlowList, List};
pub use adj_matrix::{AdjMatrix, DiFlowMat, DiMat, FlowMat, Mat};

use crate::graph::{Edge, EdgeType};

pub trait GraphStorage<W, E: Edge<W>, Ty: EdgeType> {
    fn add_vertex(&mut self) -> usize;

    fn remove_vertex(&mut self, vertex_id: usize);

    fn add_edge(&mut self, src_id: usize, dst_id: usize, edge: E) -> usize;

    fn update_edge(&mut self, src_id: usize, dst_id: usize, edge_id: usize, edge: E);

    fn remove_edge(&mut self, src_id: usize, dst_id: usize, edge_id: usize) -> Option<E>;

    fn vertex_count(&self) -> usize;

    fn vertices(&self) -> Vec<usize>;

    fn edges_between(&self, src_id: usize, dst_id: usize) -> Vec<&E>;

    fn edge_between(&self, src_id: usize, dst_id: usize, edge_id: usize) -> Option<&E> {
        self.edges_between(src_id, dst_id)
            .into_iter()
            .find(|edge| edge.get_id() == edge_id)
    }

    fn edge(&self, edge_id: usize) -> Option<&E> {
        self.edges()
            .into_iter()
            .find(|(_, _, edge)| edge.get_id() == edge_id)
            .and_then(|(_, _, edge)| Some(edge))
    }

    fn has_any_edge(&self, src_id: usize, dst_id: usize) -> bool {
        !self.edges_between(src_id, dst_id).is_empty()
    }

    fn edges(&self) -> Vec<(usize, usize, &E)> {
        if Ty::is_directed() {
            self.as_directed_edges()
        } else {
            self.as_directed_edges()
                .into_iter()
                .filter(|(src_id, dst_id, _)| src_id <= dst_id)
                .collect()
        }
    }

    fn as_directed_edges(&self) -> Vec<(usize, usize, &E)> {
        self.vertices()
        .into_iter()
        .flat_map(|src_id| {
            self.edges_from(src_id)
                .into_iter()
                .map(|(dst_id, edge)| (src_id, dst_id, edge))
                .collect::<Vec<(usize, usize, &E)>>()
        })
        .collect()
    }

    fn edges_from(&self, src_id: usize) -> Vec<(usize, &E)>;

    fn neighbors(&self, src_id: usize) -> Vec<usize>;

    fn is_directed(&self) -> bool;

    fn is_undirected(&self) -> bool {
        !self.is_directed()
    }
}
