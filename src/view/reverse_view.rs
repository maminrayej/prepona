use std::collections::HashSet;

use super::{FrozenView, SubgraphView};
use crate::common::DynIter;
use crate::provide::{Edges, Storage, Vertices};
use crate::storage::edge::Directed;

pub struct ReverseView<'a, G>
where
    G: Storage<Dir = Directed> + Vertices + Edges,
{
    inner: &'a G,

    filtered_vertices: HashSet<usize>,
    filtered_edges: HashSet<usize>,
}

impl<'a, G> ReverseView<'a, G>
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

        ReverseView {
            inner,
            filtered_vertices,
            filtered_edges,
        }
    }
}

impl<'a, G> Storage for ReverseView<'a, G>
where
    G: Storage<Dir = Directed> + Vertices + Edges,
{
    type Dir = Directed;
}

impl<'a, G> Vertices for ReverseView<'a, G>
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
                .predecessors(vt)
                .filter(|nid| self.filtered_vertices.contains(nid)),
        )
    }

    fn predecessors(&self, vt: usize) -> DynIter<'_, usize> {
        if !self.has_vt(vt) {
            panic!("View does not contain vertex with id: {}", vt);
        }

        DynIter::init(
            self.inner
                .successors(vt)
                .filter(|nid| self.filtered_vertices.contains(nid)),
        )
    }
}

impl<'a, G> Edges for ReverseView<'a, G>
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

        let (sid, did, edge) = self.inner.edge(et);

        (did, sid, edge)
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
                .outgoing_edges(vt)
                .filter(|eid| self.filtered_edges.contains(eid)),
        )
    }

    fn outgoing_edges(&self, vt: usize) -> DynIter<'_, usize> {
        if !self.has_et(vt) {
            panic!("View does not contain edge with id: {}", vt)
        }

        DynIter::init(
            self.inner
                .ingoing_edges(vt)
                .filter(|eid| self.filtered_edges.contains(eid)),
        )
    }
}

impl<'a, G> FrozenView<G> for ReverseView<'a, G>
where
    G: Storage<Dir = Directed> + Vertices + Edges,
{
    fn inner(&self) -> &G {
        self.inner
    }
}

impl<'a, G> SubgraphView<G> for ReverseView<'a, G>
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

                sid != vid && did != vid
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

#[cfg(test)]
mod tests {
    use super::ReverseView;
    use crate::provide::{Edges, Vertices};
    use crate::storage::edge::Directed;
    use crate::storage::AdjMap;
    use quickcheck_macros::quickcheck;
    use std::collections::HashSet;

    #[quickcheck]
    fn prop_reverse_view(graph: AdjMap<(), (), Directed>) {
        let view = ReverseView::init(&graph, |_| true, |_| true);

        assert_eq!(graph.vertex_count(), view.vertex_count());
        assert_eq!(graph.edge_count(), view.edge_count());

        for vid in graph.vertex_tokens() {
            assert_eq!(
                graph.successors(vid).collect::<HashSet<usize>>(),
                view.predecessors(vid).collect()
            );
            assert_eq!(
                graph.predecessors(vid).collect::<HashSet<usize>>(),
                view.successors(vid).collect()
            );
            assert_eq!(
                graph.neighbors(vid).collect::<HashSet<usize>>(),
                view.predecessors(vid).collect()
            )
        }

        for eid in graph.edge_tokens() {
            let (g_sid, g_did, g_edge) = graph.edge(eid);
            let (v_sid, v_did, v_edge) = view.edge(eid);

            assert_eq!((g_sid, g_did, g_edge), (v_did, v_sid, v_edge))
        }
    }
}
