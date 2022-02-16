use std::collections::HashSet;

use super::{FrozenView, SubgraphView};
use crate::common::DynIter;
use crate::provide::{Edges, Storage, Vertices};

pub struct GenericView<'a, G>
where
    G: Storage + Vertices + Edges,
{
    inner: &'a G,

    filtered_vertices: HashSet<usize>,
    filtered_edges: HashSet<usize>,
}

impl<'a, G> GenericView<'a, G>
where
    G: Storage + Vertices + Edges,
{
    // TODO: This method is shared between all view. DRY it!
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

        GenericView {
            inner,
            filtered_vertices,
            filtered_edges,
        }
    }
}

impl<'a, G> Storage for GenericView<'a, G>
where
    G: Storage + Vertices + Edges,
{
    type Dir = G::Dir;
}

impl<'a, G> Vertices for GenericView<'a, G>
where
    G: Storage + Vertices + Edges,
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
                .filter(|nid| self.filtered_vertices.contains(nid)),
        )
    }

    fn predecessors(&self, vt: usize) -> DynIter<'_, usize> {
        if !self.has_vt(vt) {
            panic!("View does not contain vertex with id: {}", vt);
        }

        DynIter::init(
            self.inner
                .predecessors(vt)
                .filter(|nid| self.filtered_vertices.contains(nid)),
        )
    }
}

impl<'a, G> Edges for GenericView<'a, G>
where
    G: Storage + Vertices + Edges,
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
                .filter(|eid| self.filtered_edges.contains(eid)),
        )
    }

    fn outgoing_edges(&self, vt: usize) -> DynIter<'_, usize> {
        if !self.has_vt(vt) {
            panic!("View does not contain edge with id: {}", vt)
        }

        DynIter::init(
            self.inner
                .outgoing_edges(vt)
                .filter(|eid| self.filtered_edges.contains(eid)),
        )
    }

    fn edges_between(&self, src_id: usize, dst_id: usize) -> DynIter<'_, usize> {
        if !self.has_vt(src_id) {
            panic!("View does not contain vertex with id: {}", src_id);
        } else if !self.has_vt(dst_id) {
            panic!("View does not contain vertex with id: {}", dst_id);
        }

        DynIter::init(
            self.inner
                .edges_between(src_id, dst_id)
                .filter(|eid| self.filtered_edges.contains(eid)),
        )
    }
}

impl<'a, G> FrozenView<G> for GenericView<'a, G>
where
    G: Storage + Vertices + Edges,
{
    fn inner(&self) -> &G {
        self.inner
    }
}

impl<'a, G> SubgraphView<G> for GenericView<'a, G>
where
    G: Storage + Vertices + Edges,
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
    use itertools::Itertools;
    use quickcheck::quickcheck;

    use crate::provide::{Edges, Vertices};
    use crate::storage::edge::{Directed, Direction, Undirected};
    use crate::storage::AdjMap;
    use crate::view::{GenericView, SubgraphView};

    #[test]
    fn prop_generic_view() {
        fn prop<Dir: Direction>(graph: AdjMap<(), (), Dir>) {
            let total_vertex_count = graph.vertex_count();
            let odd_vertices_count = graph.vertex_tokens().filter(|vid| *vid % 2 != 0).count();

            let mut view = GenericView::init(&graph, |vid| vid % 2 == 0, |eid| eid % 2 == 0);

            let view_vertex_count = view.vertex_count();
            let view_edge_count = view.edge_count();

            assert!(view.vertex_tokens().all(|vid| vid % 2 == 0));
            assert_eq!(view.vertex_count(), total_vertex_count - odd_vertices_count);

            for vid in view.vertex_tokens() {
                assert!(view.neighbors(vid).all(|nid| nid % 2 == 0));
                assert!(view.successors(vid).all(|nid| nid % 2 == 0));
                assert!(view.predecessors(vid).all(|nid| nid % 2 == 0));
                assert_eq!(view.vertex(vid), graph.vertex(vid));
            }

            for eid in view.edge_tokens() {
                let (sid, did, edge) = view.edge(eid);

                assert!(view.has_vt(sid));
                assert!(view.has_vt(did));
                assert_eq!(edge, graph.edge(eid).2);
            }

            // If we add back all the removed vertices and edges, view must be equal to the original graph
            graph
                .vertex_tokens()
                .filter(|vid| *vid % 2 != 0)
                .for_each(|vid| view.add_vertex_from_inner_checked(vid).unwrap());
            graph
                .edge_tokens()
                .for_each(|eid| view.add_edge_from_inner_checked(eid).unwrap());

            assert_eq!(view.vertex_count(), graph.vertex_count());
            assert_eq!(view.edge_count(), graph.edge_count());

            // If we remove them again, we must have the old view back
            graph
                .vertex_tokens()
                .filter(|vid| *vid % 2 != 0)
                .for_each(|vid| view.remove_vertex_from_view_checked(vid).unwrap());

            let to_remove_eids = graph
                .edge_tokens()
                .filter(|eid| *eid % 2 != 0 && view.has_et(*eid))
                .collect_vec();
            to_remove_eids
                .into_iter()
                .for_each(|eid| view.remove_edge_from_view_checked(eid).unwrap());

            assert_eq!(view.vertex_count(), view_vertex_count);
            assert_eq!(view.edge_count(), view_edge_count);
        }

        quickcheck(prop as fn(AdjMap<(), (), Undirected>));
        quickcheck(prop as fn(AdjMap<(), (), Directed>));
    }
}
