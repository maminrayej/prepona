use std::marker::PhantomData;

use crate::{
    provide::{
        CheckedEdges, CheckedMutEdges, CheckedVertices, Edges, InitializableStorage, MutEdges,
        MutVertices, Vertices,
    },
    storage::{
        edge::{Direction, EdgeDescriptor},
        vertex::VertexDescriptor,
    },
};

use super::GraphError;

pub struct SimpleGraph<S, V, E, Dir>
where
    V: VertexDescriptor,
    E: EdgeDescriptor,
    Dir: Direction,
    S: InitializableStorage<Dir = Dir> + Vertices<V = V, Dir = Dir> + Edges<E = E, Dir = Dir>,
{
    storage: S,

    phantom_v: PhantomData<V>,
    phantom_e: PhantomData<E>,
    phantom_dir: PhantomData<Dir>,
}

impl<S, V, E, Dir> SimpleGraph<S, V, E, Dir>
where
    V: VertexDescriptor,
    E: EdgeDescriptor,
    Dir: Direction,
    S: InitializableStorage<Dir = Dir> + Vertices<V = V, Dir = Dir> + Edges<E = E, Dir = Dir>,
{
    pub fn init() -> Self {
        SimpleGraph {
            storage: S::init(),
            phantom_v: PhantomData,
            phantom_e: PhantomData,
            phantom_dir: PhantomData,
        }
    }
}

impl<S, V, E, Dir> Vertices for SimpleGraph<S, V, E, Dir>
where
    V: VertexDescriptor,
    E: EdgeDescriptor,
    Dir: Direction,
    S: InitializableStorage<Dir = Dir> + Vertices<V = V, Dir = Dir> + Edges<E = E, Dir = Dir>,
{
    type V = V;

    type Dir = Dir;

    fn vertex(&self, vt: usize) -> &Self::V {
        self.storage.vertex(vt)
    }

    fn vertex_count(&self) -> usize {
        self.storage.vertex_count()
    }

    fn vertex_tokens(&self) -> crate::common::DynIter<'_, usize> {
        self.storage.vertex_tokens()
    }

    fn vertices(&self) -> crate::common::DynIter<'_, &Self::V> {
        self.storage.vertices()
    }

    fn neighbors(&self, vt: usize) -> crate::common::DynIter<'_, usize> {
        self.storage.neighbors(vt)
    }

    fn has_vt(&self, vt: usize) -> bool {
        self.storage.has_vt(vt)
    }

    fn successors(&self, vt: usize) -> crate::common::DynIter<'_, usize> {
        self.storage.successors(vt)
    }

    fn predecessors(&self, vt: usize) -> crate::common::DynIter<'_, usize> {
        self.storage.predecessors(vt)
    }
}

impl<S, V, E, Dir> CheckedVertices for SimpleGraph<S, V, E, Dir>
where
    V: VertexDescriptor,
    E: EdgeDescriptor,
    Dir: Direction,
    S: InitializableStorage<Dir = Dir> + Vertices<V = V, Dir = Dir> + Edges<E = E, Dir = Dir>,
{
}

impl<S, V, E, Dir> Edges for SimpleGraph<S, V, E, Dir>
where
    V: VertexDescriptor,
    E: EdgeDescriptor,
    Dir: Direction,
    S: InitializableStorage<Dir = Dir> + Vertices<V = V, Dir = Dir> + Edges<E = E, Dir = Dir>,
{
    type E = E;

    type Dir = Dir;

    fn edge(&self, et: usize) -> (usize, usize, &Self::E) {
        self.storage.edge(et)
    }

    fn edge_count(&self) -> usize {
        self.storage.edge_count()
    }

    fn edge_tokens(&self) -> crate::common::DynIter<'_, usize> {
        self.storage.edge_tokens()
    }

    fn edges(&self) -> crate::common::DynIter<'_, (usize, usize, &Self::E)> {
        self.storage.edges()
    }

    fn ingoing_edges(&self, vt: usize) -> crate::common::DynIter<'_, usize> {
        self.storage.ingoing_edges(vt)
    }

    fn outgoing_edges(&self, vt: usize) -> crate::common::DynIter<'_, usize> {
        self.storage.outgoing_edges(vt)
    }

    fn has_et(&self, et: usize) -> bool {
        self.storage.has_et(et)
    }
}

impl<S, V, E, Dir> CheckedEdges for SimpleGraph<S, V, E, Dir>
where
    V: VertexDescriptor,
    E: EdgeDescriptor,
    Dir: Direction,
    S: InitializableStorage<Dir = Dir> + Vertices<V = V, Dir = Dir> + Edges<E = E, Dir = Dir>,
{
}

impl<S, V, E, Dir> MutVertices for SimpleGraph<S, V, E, Dir>
where
    V: VertexDescriptor,
    E: EdgeDescriptor,
    Dir: Direction,
    S: InitializableStorage<Dir = Dir>
        + Vertices<V = V, Dir = Dir>
        + Edges<E = E, Dir = Dir>
        + MutVertices,
{
    fn has_free_token(&mut self) -> bool {
        self.storage.has_free_token()
    }

    fn vertex_mut(&mut self, vt: usize) -> &mut Self::V {
        self.storage.vertex_mut(vt)
    }

    fn add_vertex(&mut self, vertex: Self::V) -> usize {
        self.storage.add_vertex(vertex)
    }

    fn remove_vertex(&mut self, vt: usize) -> Self::V {
        self.storage.remove_vertex(vt)
    }
}

impl<S, V, E, Dir> MutEdges for SimpleGraph<S, V, E, Dir>
where
    V: VertexDescriptor,
    E: EdgeDescriptor,
    Dir: Direction,
    S: InitializableStorage<Dir = Dir>
        + Vertices<V = V, Dir = Dir>
        + Edges<E = E, Dir = Dir>
        + MutEdges,
{
    fn has_free_et(&mut self) -> bool {
        self.storage.has_free_et()
    }

    fn edge_mut(&mut self, et: usize) -> (usize, usize, &mut Self::E) {
        self.storage.edge_mut(et)
    }

    fn add_edge(&mut self, src_vt: usize, dst_vt: usize, edge: Self::E) -> usize {
        if self.storage.neighbors(src_vt).any(|n_vt| n_vt == dst_vt) {
            panic!("There is already an edge from {} to {}", src_vt, dst_vt);
        } else if src_vt == dst_vt {
            panic!("Adding edge from {} to itself creates a loop. Loop is not allowed in a simple graph", src_vt);
        }

        self.storage.add_edge(src_vt, dst_vt, edge)
    }

    fn remove_edge(&mut self, src_vt: usize, dst_vt: usize, et: usize) -> Self::E {
        self.storage.remove_edge(src_vt, dst_vt, et)
    }
}

impl<S, V, E, Dir> CheckedMutEdges for SimpleGraph<S, V, E, Dir>
where
    V: VertexDescriptor,
    E: EdgeDescriptor,
    Dir: Direction,
    S: InitializableStorage<Dir = Dir>
        + Vertices<V = V, Dir = Dir>
        + Edges<E = E, Dir = Dir>
        + CheckedMutEdges,
{
    fn add_edge_checked(
        &mut self,
        src_vt: usize,
        dst_vt: usize,
        edge: Self::E,
    ) -> anyhow::Result<usize> {
        if self.storage.neighbors(src_vt).any(|n_vt| n_vt == dst_vt) {
            return Err(GraphError::MultiEdge(src_vt, dst_vt).into());
        } else if src_vt == dst_vt {
            return Err(GraphError::Loop(src_vt).into());
        }

        self.storage.add_edge_checked(src_vt, dst_vt, edge)
    }
}
