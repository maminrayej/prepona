use std::collections::HashSet;

use super::{FrozenView, SubgraphView};
use crate::common::DynIter;
use crate::provide::{Edges, Storage, Vertices};
use crate::storage::edge::{Directed, Undirected};

pub struct UndirectedView<'a, G>
where
    G: Storage<Dir = Directed> + Vertices + Edges,
{
    inner: &'a G,

    filtered_vertices: HashSet<usize>,
    filtered_edges: HashSet<usize>,
}

impl<'a, G> UndirectedView<'a, G>
where
    G: Storage<Dir = Directed> + Vertices + Edges,
{
    pub fn init(
        inner: &'a G,
        vertex_filter: impl Fn(usize) -> bool,
        edge_filter: impl Fn(usize) -> bool,
    ) -> Self {
        let filtered_vertices: HashSet<usize> = inner
            .vertex_tokens()
            .filter(|&vid| vertex_filter(vid))
            .collect();

        let filtered_edges: HashSet<usize> = inner
            .edge_tokens()
            .filter(|&eid| {
                let (sid, did, _) = inner.edge(eid);

                filtered_vertices.contains(&sid)
                    && filtered_vertices.contains(&did)
                    && edge_filter(eid)
            })
            .collect();

        UndirectedView {
            inner,
            filtered_vertices,
            filtered_edges,
        }
    }
}

impl<'a, G> Storage for UndirectedView<'a, G>
where
    G: Storage<Dir = Directed> + Vertices + Edges,
{
    type Dir = Undirected;
}

impl<'a, G> Vertices for UndirectedView<'a, G>
where
    G: Storage<Dir = Directed> + Vertices + Edges,
{
    type V = G::V;

    fn has_vt(&self, vt: usize) -> bool {
        self.filtered_vertices.contains(&vt)
    }

    fn vertex(&self, vt: usize) -> &Self::V {
        if !self.has_vt(vt) {
            panic!("View does not contain vertex with id: {}", vt);
        }

        self.inner.vertex(vt)
    }

    fn vertex_count(&self) -> usize {
        self.filtered_vertices.len()
    }

    fn vertex_tokens(&self) -> DynIter<'_, usize> {
        DynIter::init(self.filtered_vertices.iter().copied())
    }

    fn vertices(&self) -> DynIter<'_, &Self::V> {
        DynIter::init(
            self.filtered_vertices
                .iter()
                .map(|vid| self.inner.vertex(*vid)),
        )
    }

    fn neighbors(&self, vt: usize) -> DynIter<'_, usize> {
        self.successors(vt)
    }

    fn successors(&self, vt: usize) -> DynIter<'_, usize> {
        if !self.has_vt(vt) {
            panic!("View does not contain vertex with id: {}", vt);
        }

        DynIter::init(
            self.inner
                .successors(vt)
                .chain(self.predecessors(vt))
                .filter(|nid| self.filtered_vertices.contains(nid)),
        )
    }

    fn predecessors(&self, vt: usize) -> DynIter<'_, usize> {
        self.successors(vt)
    }
}

impl<'a, G> Edges for UndirectedView<'a, G>
where
    G: Storage<Dir = Directed> + Vertices + Edges,
{
    type E = G::E;

    fn has_et(&self, et: usize) -> bool {
        self.filtered_edges.contains(&et)
    }

    fn edge(&self, et: usize) -> (usize, usize, &Self::E) {
        if !self.has_et(et) {
            panic!("View does not contain edge with id: {}", et)
        }

        self.inner.edge(et)
    }

    fn edge_count(&self) -> usize {
        self.filtered_edges.len()
    }

    fn edge_tokens(&self) -> DynIter<'_, usize> {
        DynIter::init(self.filtered_edges.iter().copied())
    }

    fn edges(&self) -> DynIter<'_, (usize, usize, &Self::E)> {
        DynIter::init(self.filtered_edges.iter().map(|eid| self.inner.edge(*eid)))
    }

    fn ingoing_edges(&self, vt: usize) -> DynIter<'_, usize> {
        if !self.has_vt(vt) {
            panic!("View does not contain edge with id: {}", vt)
        }

        DynIter::init(
            self.inner
                .ingoing_edges(vt)
                .chain(self.inner.outgoing_edges(vt))
                .filter(|eid| self.filtered_edges.contains(eid)),
        )
    }

    fn outgoing_edges(&self, vt: usize) -> DynIter<'_, usize> {
        self.ingoing_edges(vt)
    }
}

impl<'a, G> FrozenView<G> for UndirectedView<'a, G>
where
    G: Storage<Dir = Directed> + Vertices + Edges,
{
    fn inner(&self) -> &G {
        self.inner
    }
}

impl<'a, G> SubgraphView<G> for UndirectedView<'a, G>
where
    G: Storage<Dir = Directed> + Vertices + Edges,
{
    fn add_vertex_from_inner(&mut self, vid: usize) {
        self.filtered_vertices.insert(vid);
    }

    fn remove_vertex_from_view(&mut self, vid: usize) {
        self.filtered_vertices.remove(&vid).then(|| {
            self.filtered_edges.retain(|et| {
                let (sid, did, _) = self.inner.edge(*et);

                sid == vid || did == vid
            });
        });
    }

    fn add_edge_from_inner(&mut self, eid: usize) {
        let (sid, did, _) = self.inner.edge(eid);

        self.filtered_vertices.insert(sid);
        self.filtered_vertices.insert(did);
        self.filtered_edges.insert(eid);
    }

    fn remove_edge_from_view(&mut self, eid: usize) {
        self.filtered_edges.remove(&eid);
    }
}
