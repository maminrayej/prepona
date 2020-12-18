use std::marker::PhantomData;

use crate::{
    graph::EdgeDir,
    prelude::{Edge, Edges, Graph, Neighbors, Vertices},
};

use super::{AsFrozenSubgraph, AsSubgraph};

pub struct Subgraph<'a, W, E, Dir, G>
where
    E: Edge<W>,
    Dir: EdgeDir,
    G: Graph<W, E, Dir>,
{
    graph: &'a G,

    edges: Vec<(usize, usize, usize)>,
    vertex_ids: Vec<usize>,

    phantom_w: PhantomData<W>,
    phantom_e: PhantomData<E>,
    phantom_dir: PhantomData<Dir>,
}

impl<'a, W, E, Dir, G> Subgraph<'a, W, E, Dir, G>
where
    E: Edge<W>,
    Dir: EdgeDir,
    G: Graph<W, E, Dir> + Edges<W, E> + Neighbors,
{
    pub fn init(graph: &'a G, edges: Vec<(usize, usize, usize)>, vertex_ids: Vec<usize>) -> Self {
        Subgraph {
            graph,
            edges,
            vertex_ids,

            phantom_w: PhantomData,
            phantom_e: PhantomData,
            phantom_dir: PhantomData,
        }
    }
}

impl<'a, W, E, Dir, G> Neighbors for Subgraph<'a, W, E, Dir, G>
where
    E: Edge<W>,
    Dir: EdgeDir,
    G: Graph<W, E, Dir> + Edges<W, E> + Neighbors,
{
    fn neighbors(&self, src_id: usize) -> Vec<usize> {
        self.graph
            .neighbors(src_id)
            .into_iter()
            .filter(|vertex_id| self.vertex_ids.contains(vertex_id))
            .collect()
    }
}

impl<'a, W, E, Dir, G> Vertices for Subgraph<'a, W, E, Dir, G>
where
    E: Edge<W>,
    Dir: EdgeDir,
    G: Graph<W, E, Dir>,
{
    fn vertices(&self) -> Vec<usize> {
        self.vertex_ids.clone()
    }
}

impl<'a, W, E, Dir, G> Edges<W, E> for Subgraph<'a, W, E, Dir, G>
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
}

impl<'a, W, E, Dir, G> AsFrozenSubgraph<W, E> for Subgraph<'a, W, E, Dir, G>
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

impl<'a, W, E, Dir, G> AsSubgraph<W, E> for Subgraph<'a, W, E, Dir, G>
where
    E: Edge<W>,
    Dir: EdgeDir,
    G: Graph<W, E, Dir> + Vertices + Neighbors + Edges<W, E>,
{
    fn remove_edge(&mut self, _: usize, _: usize, edge_id: usize) {
        self.edges.retain(|(_, _, e_id)| *e_id != edge_id);
    }

    fn remove_vertex(&mut self, vertex_id: usize) {
        self.edges
            .retain(|(src_id, dst_id, _)| *src_id != vertex_id && *dst_id != vertex_id);
        self.vertex_ids.retain(|v_id| *v_id != vertex_id);
    }

    fn add_vertex_from_graph(&mut self, vertex_id: usize) {
        if self.graph.vertices().contains(&vertex_id) {
            self.vertex_ids.push(vertex_id)
        }
    }

    fn add_edge_from_graph(&mut self, src_id: usize, dst_id: usize, edge_id: usize) {
        if !self.has_edge(edge_id) && self.graph.edge_between(src_id, dst_id, edge_id).is_some() {
            self.edges.push((src_id, dst_id, edge_id))
        }
    }
}
