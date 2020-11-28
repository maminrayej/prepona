use std::any::Any;
use std::marker::PhantomData;

use crate::graph::{DefaultEdge, Edge, EdgeType, FlowEdge};
use crate::provide;
use crate::storage::{FlowMat, GraphStorage, Mat, List, FlowList};

pub type MatGraph<W, Ty> = SimpleGraph<W, DefaultEdge<W>, Ty, Mat<W, Ty>>;
pub type ListGraph<W, Ty> = SimpleGraph<W, DefaultEdge<W>, Ty, List<W, Ty>>;
// pub type DiMatGraph<W> = SimpleGraph<W, DefaultEdge<W>, DirectedEdge, DiMat<W>>;

pub type FlowMatGraph<W, Ty> = SimpleGraph<W, FlowEdge<W>, Ty, FlowMat<W>>;
pub type FlowListGraph<W, Ty> = SimpleGraph<W, DefaultEdge<W>, Ty, FlowList<W, Ty>>;
// pub type DiFlowMatGraph<W> = SimpleGraph<W, FlowEdge<W>, DirectedEdge, DiFlowMat<W>>;

pub struct SimpleGraph<W, E: Edge<W>, Ty: EdgeType, S: GraphStorage<W, E, Ty>> {
    // Backend storage to store graph data
    storage: S,

    phantom_w: PhantomData<W>,
    phantom_e: PhantomData<E>,
    phantom_ty: PhantomData<Ty>,
}

impl<W: Any + Copy, E: Edge<W>, Ty: EdgeType, S: GraphStorage<W, E, Ty>> SimpleGraph<W, E, Ty, S> {
    pub fn init(storage: S) -> Self {
        SimpleGraph {
            storage,

            phantom_e: PhantomData,
            phantom_w: PhantomData,
            phantom_ty: PhantomData,
        }
    }
}

impl<W, E: Edge<W>, Ty: EdgeType, S: GraphStorage<W, E, Ty>> provide::Neighbors
    for SimpleGraph<W, E, Ty, S>
{
    fn neighbors(&self, src_id: usize) -> Vec<usize> {
        self.storage.neighbors(src_id)
    }
}

impl<W, E: Edge<W>, Ty: EdgeType, S: GraphStorage<W, E, Ty>> provide::Vertices
    for SimpleGraph<W, E, Ty, S>
{
    fn vertices(&self) -> Vec<usize> {
        self.storage.vertices()
    }

    fn vertex_count(&self) -> usize {
        self.storage.vertex_count()
    }
}

impl<W, E: Edge<W>, Ty: EdgeType, S: GraphStorage<W, E, Ty>> provide::Edges<W, E>
    for SimpleGraph<W, E, Ty, S>
{
    fn edges_from(&self, src_id: usize) -> Vec<(usize, &E)> {
        self.storage.edges_from(src_id)
    }

    fn edges_between(&self, src_id: usize, dst_id: usize) -> Vec<&E> {
        self.storage.edges_between(src_id, dst_id)
    }

    fn edge_between(&self, src_id: usize, dst_id: usize, edge_id: usize) -> Option<&E> {
        self.storage.edge_between(src_id, dst_id, edge_id)
    }

    fn edge(&self, edge_id: usize) -> Option<&E> {
        self.storage.edge(edge_id)
    }

    fn has_any_edge(&self, src_id: usize, dst_id: usize) -> bool {
        self.storage.has_any_edge(src_id, dst_id)
    }

    fn edges(&self) -> Vec<(usize, usize, &E)> {
        self.storage.edges()
    }

    fn as_directed_edges(&self) -> Vec<(usize, usize, &E)> {
        self.storage.as_directed_edges()
    }

    fn edges_count(&self) -> usize {
        self.storage.edges().len()
    }
}

impl<W, E: Edge<W>, Ty: EdgeType, S: GraphStorage<W, E, Ty>> provide::Graph<W, E, Ty>
    for SimpleGraph<W, E, Ty, S>
{
    fn add_vertex(&mut self) -> usize {
        self.storage.add_vertex()
    }

    fn remove_vertex(&mut self, vertex_id: usize) {
        self.storage.remove_vertex(vertex_id);
    }

    fn add_edge(&mut self, src_id: usize, dst_id: usize, edge: E) -> usize {
        if src_id == dst_id {
            panic!("Can not create loop in simple graph")
        }

        if self.storage.has_any_edge(src_id, dst_id) {
            panic!("Can not add multiple edges between two vertices in simple graph");
        }

        self.storage.add_edge(src_id, dst_id, edge)
    }

    fn update_edge(&mut self, src_id: usize, dst_id: usize, edge_id: usize, edge: E) {
        self.storage.update_edge(src_id, dst_id, edge_id, edge);
    }

    fn remove_edge(&mut self, src_id: usize, dst_id: usize, edge_id: usize) -> Option<E> {
        self.storage.remove_edge(src_id, dst_id, edge_id)
    }
}

impl<W, E: Edge<W>, Ty: EdgeType, S: GraphStorage<W, E, Ty>> provide::Direction
    for SimpleGraph<W, E, Ty, S>
{
    fn is_directed() -> bool {
        Ty::is_directed()
    }

    fn is_undirected() -> bool {
        Ty::is_undirected()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::provide::*;

    #[test]
    #[should_panic(expected = "Can not create loop in simple graph")]
    fn add_loop() {
        // Given: An empty graph.
        let mut graph = MatGraph::init(Mat::<usize>::init());

        // When: Adding an edge from a vertex to itself.
        graph.add_edge(0, 0, 1.into());

        // Then: Code should panic.
    }

    #[test]
    #[should_panic(expected = "Can not add multiple edges between two vertices in simple graph")]
    fn add_multiple_edge() {
        // Given: Graph
        //
        //      a  --- b
        //
        let mut graph = MatGraph::init(Mat::<usize>::init());
        let a = graph.add_vertex();
        let b = graph.add_vertex();
        graph.add_edge(a, b, 1.into());

        // When: Trying to add another edge between a and b.
        graph.add_edge(a, b, 1.into());

        // Then: Code should panic.
    }
}
