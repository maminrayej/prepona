use std::marker::PhantomData;

use super::GraphError;
use crate::common::DynIter;
use crate::provide::{Edges, InitializableStorage, MutEdges, MutVertices, Storage, Vertices};
use crate::storage::edge::{Direction, EdgeDescriptor};
use crate::storage::vertex::VertexDescriptor;
use crate::storage::AdjMap;

pub type SimpleGraphMap<V, E, DIR> = SimpleGraph<AdjMap<V, E, DIR>, V, E, DIR>;

#[derive(Debug)]
pub struct SimpleGraph<S, V, E, Dir>
where
    V: VertexDescriptor,
    E: EdgeDescriptor,
    Dir: Direction,
    S: Storage<Dir = Dir> + InitializableStorage + Vertices<V = V> + Edges<E = E>,
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
    S: Storage<Dir = Dir> + InitializableStorage + Vertices<V = V> + Edges<E = E>,
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

impl<S, V, E, Dir> Storage for SimpleGraph<S, V, E, Dir>
where
    V: VertexDescriptor,
    E: EdgeDescriptor,
    Dir: Direction,
    S: Storage<Dir = Dir> + InitializableStorage + Vertices<V = V> + Edges<E = E>,
{
    type Dir = Dir;
}

impl<S, V, E, Dir> InitializableStorage for SimpleGraph<S, V, E, Dir>
where
    V: VertexDescriptor,
    E: EdgeDescriptor,
    Dir: Direction,
    S: Storage<Dir = Dir> + InitializableStorage + Vertices<V = V> + Edges<E = E>,
{
    fn init() -> Self {
        SimpleGraph::init()
    }
}

impl<S, V, E, Dir> Vertices for SimpleGraph<S, V, E, Dir>
where
    V: VertexDescriptor,
    E: EdgeDescriptor,
    Dir: Direction,
    S: Storage<Dir = Dir> + InitializableStorage + Vertices<V = V> + Edges<E = E>,
{
    type V = V;

    fn has_vt(&self, vt: usize) -> bool {
        self.storage.has_vt(vt)
    }

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

    fn successors(&self, vt: usize) -> crate::common::DynIter<'_, usize> {
        self.storage.successors(vt)
    }

    fn predecessors(&self, vt: usize) -> crate::common::DynIter<'_, usize> {
        self.storage.predecessors(vt)
    }
}

impl<S, V, E, Dir> Edges for SimpleGraph<S, V, E, Dir>
where
    V: VertexDescriptor,
    E: EdgeDescriptor,
    Dir: Direction,
    S: Storage<Dir = Dir> + InitializableStorage + Vertices<V = V> + Edges<E = E>,
{
    type E = E;

    fn has_et(&self, et: usize) -> bool {
        self.storage.has_et(et)
    }

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

    fn edges_between(&self, src_id: usize, dst_id: usize) -> crate::common::DynIter<'_, usize> {
        if src_id != dst_id {
            DynIter::init([].into_iter())
        } else {
            self.storage.edges_between(src_id, dst_id)
        }
    }
}

impl<S, V, E, Dir> MutVertices for SimpleGraph<S, V, E, Dir>
where
    V: VertexDescriptor,
    E: EdgeDescriptor,
    Dir: Direction,
    S: Storage<Dir = Dir> + InitializableStorage + Vertices<V = V> + Edges<E = E> + MutVertices,
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
    S: Storage<Dir = Dir> + InitializableStorage + Vertices<V = V> + Edges<E = E> + MutEdges,
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

    fn remove_edge(&mut self, src_vt: usize, dst_vt: usize, et: usize) -> Self::E {
        self.storage.remove_edge(src_vt, dst_vt, et)
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use std::marker::PhantomData;

    use itertools::Itertools;
    use quickcheck::Arbitrary;
    use rand::{thread_rng, Rng};

    use crate::provide::{Edges, InitializableStorage, MutEdges, MutVertices, Storage, Vertices};
    use crate::storage::edge::{Direction, EdgeDescriptor};
    use crate::storage::vertex::VertexDescriptor;

    use super::SimpleGraph;

    impl<S, V, E, Dir> Clone for SimpleGraph<S, V, E, Dir>
    where
        V: VertexDescriptor,
        E: EdgeDescriptor,
        Dir: Direction,
        S: Storage<Dir = Dir> + InitializableStorage + Vertices<V = V> + Edges<E = E> + Clone,
    {
        fn clone(&self) -> Self {
            Self {
                storage: self.storage.clone(),
                phantom_v: self.phantom_v.clone(),
                phantom_e: self.phantom_e.clone(),
                phantom_dir: self.phantom_dir.clone(),
            }
        }
    }

    impl<S, V, E, Dir> Arbitrary for SimpleGraph<S, V, E, Dir>
    where
        V: VertexDescriptor + Arbitrary,
        E: EdgeDescriptor + Arbitrary,
        Dir: Direction + Arbitrary,
        S: Storage<Dir = Dir>
            + InitializableStorage
            + Vertices<V = V>
            + Edges<E = E>
            + MutVertices
            + MutEdges
            + Arbitrary,
    {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let vertex_count = usize::arbitrary(g) % 20;

            let mut rng = thread_rng();
            let edge_probability = rng.gen::<f64>() * rng.gen::<f64>();

            let mut adj_map = S::init();

            let vts: Vec<usize> = (0..vertex_count)
                .map(|_| adj_map.add_vertex(V::arbitrary(g)))
                .collect();

            vts.iter().cartesian_product(vts.iter()).for_each(|(i, j)| {
                let p = rng.gen::<f64>();

                if i < j && p <= edge_probability {
                    let num_of_edges = (usize::arbitrary(g) % 3) + 1;

                    for _ in 0..num_of_edges {
                        adj_map.add_edge(*i, *j, E::arbitrary(g));
                    }
                }
            });

            SimpleGraph {
                storage: adj_map,
                phantom_v: PhantomData,
                phantom_e: PhantomData,
                phantom_dir: PhantomData,
            }
        }

        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            let mut graph_odd = Self::init();
            let mut graph_even = Self::init();

            let mut vt_map = HashMap::new();

            for (index, vt) in self.vertex_tokens().enumerate() {
                let new_vt = if index % 2 == 0 {
                    graph_odd.add_vertex(self.vertex(vt).clone())
                } else {
                    graph_even.add_vertex(self.vertex(vt).clone())
                };

                vt_map.insert(vt, new_vt);
            }

            for (src_vt, dst_vt, edge) in self.edges() {
                let new_src_vt = vt_map[&src_vt];
                let new_dst_vt = vt_map[&dst_vt];

                if graph_odd.has_vt(new_src_vt) && graph_odd.has_vt(new_dst_vt) {
                    graph_odd.add_edge(new_src_vt, new_dst_vt, edge.clone());
                } else if graph_even.has_vt(new_src_vt) && graph_even.has_vt(new_dst_vt) {
                    graph_even.add_edge(new_src_vt, new_dst_vt, edge.clone());
                }
            }

            let before_count = self.vertex_count();

            Box::new(
                [graph_odd, graph_even]
                    .into_iter()
                    .filter(move |graph| graph.vertex_count() < before_count),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use rand::prelude::IteratorRandom;
    use rand::thread_rng;

    use crate::graph::SimpleGraphMap;
    use crate::provide::{MutEdges, Vertices};
    use crate::storage::edge::{Directed, Direction, Undirected};
    use crate::test_utils::get_non_duplicate;

    #[should_panic]
    #[test]
    fn prop_add_loop() {
        fn prop<Dir: Direction>(mut graph: SimpleGraphMap<(), (), Dir>) {
            if graph.vertex_count() == 0 {
                return;
            }

            let vertex_id = graph.vertex_tokens().choose(&mut thread_rng()).unwrap();

            graph.add_edge(vertex_id, vertex_id, ());
        }

        quickcheck::quickcheck(prop as fn(SimpleGraphMap<(), (), Undirected>));
        quickcheck::quickcheck(prop as fn(SimpleGraphMap<(), (), Directed>));
    }

    #[test]
    fn prop_add_loop_checked() {
        fn prop<Dir: Direction>(mut graph: SimpleGraphMap<(), (), Dir>) -> bool {
            if graph.vertex_count() == 0 {
                return true;
            }

            let vertex_id = graph.vertex_tokens().choose(&mut thread_rng()).unwrap();

            graph.add_edge_checked(vertex_id, vertex_id, ()).is_err()
        }

        quickcheck::quickcheck(prop as fn(SimpleGraphMap<(), (), Undirected>) -> bool);
        quickcheck::quickcheck(prop as fn(SimpleGraphMap<(), (), Directed>) -> bool);
    }

    #[should_panic]
    #[test]
    fn prop_add_multi_edge() {
        fn prop<Dir: Direction>(mut graph: SimpleGraphMap<(), (), Dir>) {
            if graph.vertex_count() < 2 {
                return;
            }

            let vertex_ids = get_non_duplicate(graph.vertex_tokens(), 2);

            let src_vid = vertex_ids[0];
            let dst_vid = vertex_ids[1];

            if graph.neighbors(src_vid).contains(&dst_vid) {
                graph.add_edge(src_vid, dst_vid, ());
            } else {
                graph.add_edge(src_vid, dst_vid, ());
                graph.add_edge(src_vid, dst_vid, ());
            }
        }

        quickcheck::quickcheck(prop as fn(SimpleGraphMap<(), (), Undirected>));
        quickcheck::quickcheck(prop as fn(SimpleGraphMap<(), (), Directed>));
    }

    #[test]
    fn prop_add_multi_edge_checked() {
        fn prop<Dir: Direction>(mut graph: SimpleGraphMap<(), (), Dir>) -> bool {
            if graph.vertex_count() < 2 {
                return true;
            }

            let vertex_ids = graph.vertex_tokens().choose_multiple(&mut thread_rng(), 2);

            let src_vid = vertex_ids[0];
            let dst_vid = vertex_ids[1];

            if graph.neighbors(src_vid).contains(&dst_vid) {
                graph.add_edge_checked(src_vid, dst_vid, ()).is_err()
            } else {
                graph.add_edge_checked(src_vid, dst_vid, ()).is_ok()
                    && graph.add_edge_checked(src_vid, dst_vid, ()).is_err()
            }
        }

        quickcheck::quickcheck(prop as fn(SimpleGraphMap<(), (), Undirected>) -> bool);
        quickcheck::quickcheck(prop as fn(SimpleGraphMap<(), (), Directed>) -> bool);
    }
}
