use super::{Edges, MutEdges, MutVertices, Vertices};

pub trait FrozenView<const DIR: bool>: Vertices + Edges {}

pub trait View<const DIR: bool>: FrozenView<DIR> {
    fn hide_vertices(&mut self, vertex_filter: impl Fn(usize) -> bool);

    fn hide_edges(&mut self, edge_filter: impl Fn(usize) -> bool);

    fn add_vertex_from_graph(&mut self, vt: usize);

    fn add_edge_from_graph(&mut self, et: usize);

    fn remove_vertex_from_view(&mut self, vt: usize);

    fn remove_edge_from_view(&mut self, et: usize);
}

pub trait MutView<const DIR: bool>: View<DIR> + MutVertices + MutEdges {}

pub trait AsUndirected<FV>
where
    FV: FrozenView<false>,
{
    fn as_undirected(
        &self,
        vertex_filter: impl Fn(usize) -> bool,
        edge_filter: impl Fn(usize) -> bool,
    ) -> FV;
}

pub trait AsDirected<FV>
where
    FV: FrozenView<true>,
{
    fn as_directed(
        &self,
        vertex_filter: impl Fn(usize) -> bool,
        edge_filter: impl Fn(usize) -> bool,
    ) -> FV;
}
