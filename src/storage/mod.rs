mod adj_list;
mod adj_matrix;

pub use adj_list::{AdjList, DiFlowList, DiList, FlowList, List};
pub use adj_matrix::{AdjMatrix, DiFlowMat, DiMat, FlowMat, Mat};

use crate::graph::{Edge, EdgeType};

pub trait GraphStorage<W, E: Edge<W>, Ty: EdgeType> {
    fn add_vertex(&mut self) -> usize;

    fn remove_vertex(&mut self, vertex_id: usize);

    fn add_edge(&mut self, src_id: usize, dst_id: usize, edge: E);

    fn update_edge(&mut self, src_id: usize, dst_id: usize, edge: E);

    fn remove_edge(&mut self, src_id: usize, dst_id: usize) -> E;

    fn vertex_count(&self) -> usize;

    fn vertices(&self) -> Vec<usize>;

    fn edge(&self, src_id: usize, dst_id: usize) -> Option<&E>;

    fn has_edge(&self, src_id: usize, dst_id: usize) -> bool {
        self.edge(src_id, dst_id).is_some()
    }

    fn edges(&self) -> Vec<(usize, usize, &E)>;

    fn edges_from(&self, src_id: usize) -> Vec<(usize, &E)>;

    fn neighbors(&self, src_id: usize) -> Vec<usize>;

    fn is_directed(&self) -> bool;

    fn is_undirected(&self) -> bool {
        !self.is_directed()
    }
}
