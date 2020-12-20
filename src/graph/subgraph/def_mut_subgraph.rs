use std::{collections::HashSet, marker::PhantomData};

use crate::{
    graph::EdgeDir,
    prelude::{Edge, Edges, Graph, Neighbors, Vertices},
};

use super::{AsFrozenSubgraph, AsMutSubgraph, AsSubgraph};

pub struct MutSubgraph<'a, W, E: Edge<W>, Dir: EdgeDir, G: Graph<W, E, Dir>> {
    graph: &'a mut G,

    edges: Vec<(usize, usize, usize)>,
    vertex_ids: HashSet<usize>,

    phantom_w: PhantomData<W>,
    phantom_e: PhantomData<E>,
    phantom_dir: PhantomData<Dir>,
}

impl<'a, W, E, Dir, G> MutSubgraph<'a, W, E, Dir, G>
where
    E: Edge<W>,
    Dir: EdgeDir,
    G: Graph<W, E, Dir> + Edges<W, E> + Neighbors,
{
    pub fn init(
        graph: &'a mut G,
        edges: Vec<(usize, usize, usize)>,
        vertex_ids: HashSet<usize>,
    ) -> Self {
        MutSubgraph {
            graph,
            edges,
            vertex_ids,

            phantom_w: PhantomData,
            phantom_e: PhantomData,
            phantom_dir: PhantomData,
        }
    }
}

impl<'a, W, E, Dir, G> Neighbors for MutSubgraph<'a, W, E, Dir, G>
where
    E: Edge<W>,
    Dir: EdgeDir,
    G: Graph<W, E, Dir> + Edges<W, E> + Neighbors,
{
    fn neighbors(&self, src_id: usize) -> Vec<usize> {
        self.graph
            .neighbors(src_id)
            .into_iter()
            .filter(|vertex_id| self.has_vertex(*vertex_id))
            .collect()
    }
}

impl<'a, W, E, Dir, G> Vertices for MutSubgraph<'a, W, E, Dir, G>
where
    E: Edge<W>,
    Dir: EdgeDir,
    G: Graph<W, E, Dir>,
{
    fn vertices(&self) -> Vec<usize> {
        self.vertex_ids.iter().copied().collect()
    }

    fn contains_vertex(&self, vertex_id: usize) -> bool {
        self.vertex_ids.contains(&vertex_id)
    }
}

impl<'a, W, E, Dir, G> Edges<W, E> for MutSubgraph<'a, W, E, Dir, G>
where
    E: Edge<W>,
    Dir: EdgeDir,
    G: Graph<W, E, Dir> + Edges<W, E> + Neighbors,
{
    fn edges_from(&self, src_id: usize) -> Vec<(usize, &E)> {
        self.graph
            .edges_from(src_id)
            .into_iter()
            .filter(|(dst_id, edge)| self.has_vertex(*dst_id) && self.has_edge(edge.get_id()))
            .collect()
    }

    fn edges_between(&self, src_id: usize, dst_id: usize) -> Vec<&E> {
        self.graph
            .edges_from(src_id)
            .into_iter()
            .filter_map(|(d_id, edge)| if d_id == dst_id { Some(edge) } else { None })
            .collect()
    }

    fn edge_between(&self, src_id: usize, dst_id: usize, edge_id: usize) -> Option<&E> {
        self.edges_between(src_id, dst_id)
            .into_iter()
            .find(|edge| edge.get_id() == edge_id)
    }

    fn edge(&self, edge_id: usize) -> Option<&E> {
        self.edges().into_iter().find_map(|(_, _, edge)| {
            if edge.get_id() == edge_id {
                Some(edge)
            } else {
                None
            }
        })
    }

    fn has_any_edge(&self, src_id: usize, dst_id: usize) -> bool {
        !self.edges_between(src_id, dst_id).is_empty()
    }

    fn edges(&self) -> Vec<(usize, usize, &E)> {
        if Dir::is_directed() {
            self.as_directed_edges()
        } else {
            self.as_directed_edges()
                .into_iter()
                .filter(|(src_id, dst_id, _)| src_id <= dst_id)
                .collect()
        }
    }

    fn as_directed_edges(&self) -> Vec<(usize, usize, &E)> {
        self.graph
            .as_directed_edges()
            .into_iter()
            .filter(|(_, _, edge)| self.has_edge(edge.get_id()))
            .collect()
    }

    fn edges_count(&self) -> usize {
        self.edges.len()
    }

    fn contains_edge(&self, edge_id: usize) -> bool {
        self.edges
            .iter()
            .find(|(_, _, e_id)| *e_id == edge_id)
            .is_some()
    }
}

impl<'a, W, E, Dir, G> AsFrozenSubgraph<W, E> for MutSubgraph<'a, W, E, Dir, G>
where
    E: Edge<W>,
    Dir: EdgeDir,
    G: Graph<W, E, Dir> + Edges<W, E> + Neighbors,
{
    fn has_vertex(&self, vertex_id: usize) -> bool {
        self.vertex_ids.contains(&vertex_id)
    }

    fn has_edge(&self, edge_id: usize) -> bool {
        self.edges
            .iter()
            .find(|(_, _, e_id)| *e_id == edge_id)
            .is_some()
    }
}

impl<'a, W, E, Dir, G> AsSubgraph<W, E> for MutSubgraph<'a, W, E, Dir, G>
where
    E: Edge<W>,
    Dir: EdgeDir,
    G: Graph<W, E, Dir> + Vertices + Neighbors + Edges<W, E>,
{
    fn remove_edge(&mut self, _: usize, _: usize, edge_id: usize) {
        self.edges.retain(|(_, _, e_id)| *e_id != edge_id)
    }

    fn remove_vertex(&mut self, vertex_id: usize) {
        self.vertex_ids.retain(|v_id| *v_id != vertex_id);
        self.edges
            .retain(|(src_id, dst_id, _)| *src_id != vertex_id && *dst_id != vertex_id);
    }

    fn add_vertex_from_graph(&mut self, vertex_id: usize) {
        if self.graph.vertices().contains(&vertex_id) {
            self.vertex_ids.insert(vertex_id);
        }
    }

    fn add_edge_from_graph(&mut self, src_id: usize, dst_id: usize, edge_id: usize) {
        if !self.has_edge(edge_id) && self.graph.edge_between(src_id, dst_id, edge_id).is_some() {
            self.edges.push((src_id, dst_id, edge_id))
        }
    }
}

impl<'a, W, E, Dir, G> AsMutSubgraph<W, E> for MutSubgraph<'a, W, E, Dir, G>
where
    E: Edge<W>,
    Dir: EdgeDir,
    G: Graph<W, E, Dir> + Vertices + Neighbors + Edges<W, E>,
{
    fn remove_vertex_from_graph(&mut self, vertex_id: usize) {
        self.graph.remove_vertex(vertex_id);
        self.remove_vertex(vertex_id);
    }

    fn remove_edge_from_graph(&mut self, src_id: usize, dst_id: usize, edge_id: usize) {
        self.graph.remove_edge(src_id, dst_id, edge_id);
        self.remove_edge(src_id, dst_id, edge_id);
    }

    fn add_vertex(&mut self) -> usize {
        let vertex_id = self.graph.add_vertex();
        self.vertex_ids.insert(vertex_id);

        vertex_id
    }

    fn add_edge(&mut self, src_id: usize, dst_id: usize, edge: E) -> usize {
        let edge_id = self.graph.add_edge(src_id, dst_id, edge);

        self.edges.push((src_id, dst_id, edge_id));

        self.vertex_ids.insert(src_id);
        self.vertex_ids.insert(dst_id);

        edge_id
    }
}
