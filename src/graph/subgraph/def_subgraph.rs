use std::{collections::HashSet, marker::PhantomData};

use anyhow::Result;

use crate::{
    graph::{error::Error, EdgeDir},
    prelude::{Edge, Edges, Graph, Neighbors, Vertices},
};

use super::{AsFrozenSubgraph, AsSubgraph};

// TODO: add verification to subgraph constructors.

/// Default implementation of [`AsSubgraph`](crate::graph::subgraph::AsSubgraph) trait.
///
/// ## Generic Parameters
/// * `W`: **W**eight type associated with edges.
/// * `E`: **E**dge type that subgraph uses.
/// * `Dir`: **Dir**ection of edges: [`Directed`](crate::graph::DirectedEdge) or [`Undirected`](crate::graph::UndirectedEdge).
/// * `G`: **G**raph type that subgraph is representing.
pub struct Subgraph<'a, W, E, Dir, G>
where
    E: Edge<W>,
    Dir: EdgeDir,
    G: Graph<W, E, Dir>,
{
    graph: &'a G,

    edges: Vec<(usize, usize, usize)>,
    vertex_ids: HashSet<usize>,

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
    /// Initializes a subgraph with provided `edges` and `vertex_ids`.
    ///
    /// # Arguments
    /// * `graph`: Graph that this subgraph is representing.
    /// * `edges`: Edges present in the subgraph in the format of (`src_id`, `dst_id`, `edge_id`).
    /// * `vertex_ids`: Vertices present in the subgraph.
    ///
    /// # Returns
    /// An initialized subgraph containing the provided edges and vertices.
    pub fn init(
        graph: &'a G,
        edges: Vec<(usize, usize, usize)>,
        vertex_ids: HashSet<usize>,
    ) -> Self {
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
    /// # Arguments:
    /// `src_id`: Id of the source vertex.
    ///
    /// # Returns
    /// * `Err`: If vertex with id: `src_id` is not present in the subgraph.
    /// * `Ok`: Containing Id of vertices accessible from source vertex using one edge.
    fn neighbors(&self, src_id: usize) -> Result<Vec<usize>> {
        if !self.contains_vertex(src_id) {
            Err(Error::new_vnf(src_id))?
        } else {
            Ok(self.neighbors_unchecked(src_id))
        }
    }

    /// # Arguments:
    /// `src_id`: Id of the source vertex.
    ///
    /// # Returns
    /// Id of vertices accessible from source vertex using one edge.
    fn neighbors_unchecked(&self, src_id: usize) -> Vec<usize> {
        self.edges
            .iter()
            .filter_map(|(s_id, dst_id, _)| if *s_id == src_id { Some(*dst_id) } else { None })
            .collect()
    }
}

impl<'a, W, E, Dir, G> Vertices for Subgraph<'a, W, E, Dir, G>
where
    E: Edge<W>,
    Dir: EdgeDir,
    G: Graph<W, E, Dir>,
{
    /// # Returns
    /// Id of vertices that are present in the graph.
    fn vertices(&self) -> Vec<usize> {
        self.vertex_ids.iter().copied().collect()
    }

    /// # Returns
    /// * `true`: If subgraph contains the vertex with id: `vertex_id`.
    /// * `false`: Otherwise.
    fn contains_vertex(&self, vertex_id: usize) -> bool {
        self.vertex_ids.contains(&vertex_id)
    }
}

impl<'a, W, E, Dir, G> Edges<W, E> for Subgraph<'a, W, E, Dir, G>
where
    E: Edge<W>,
    Dir: EdgeDir,
    G: Graph<W, E, Dir> + Edges<W, E> + Neighbors,
{
    /// # Arguments
    /// `src_id`: Id of the source vertex.
    ///
    /// # Returns
    /// * `Err`: If vertex with id: `src_id` is not present in the subgraph.
    /// * `Ok`: Containing all edges from the source vertex in the format of: (`dst_id`, `edge`)
    fn edges_from(&self, src_id: usize) -> Result<Vec<(usize, &E)>> {
        if !self.contains_vertex(src_id) {
            Err(Error::new_vnf(src_id))?
        } else {
            Ok(self.edges_from_unchecked(src_id))
        }
    }

    /// # Arguments
    /// `src_id`: Id of the source vertex.
    ///
    /// # Returns
    /// * All edges from the source vertex in the format of: (`dst_id`, `edge`)
    fn edges_from_unchecked(&self, src_id: usize) -> Vec<(usize, &E)> {
        self.graph
            .edges_from_unchecked(src_id)
            .into_iter()
            .filter(|(dst_id, edge)| {
                self.contains_vertex(*dst_id) && self.contains_edge(edge.get_id())
            })
            .collect()
    }

    /// # Arguments
    /// * `src_id`: Id of source vertex.
    /// * `dst_id`: Id of destination vertex.
    ///
    /// # Returns
    /// * `Err`: If either `src_id` or `dst_id` is not present in the subgraph.
    /// * `Ok`: Containing edges from source vertex to destination vertex.
    fn edges_between(&self, src_id: usize, dst_id: usize) -> Result<Vec<&E>> {
        if !self.contains_vertex(src_id) {
            Err(Error::new_vnf(src_id))?
        } else if !self.contains_vertex(dst_id) {
            Err(Error::new_vnf(dst_id))?
        } else {
            Ok(self.edges_between_unchecked(src_id, dst_id))
        }
    }

    /// # Arguments
    /// * `src_id`: Id of source vertex.
    /// * `dst_id`: Id of destination vertex.
    ///
    /// # Returns
    /// Edges from source vertex to destination vertex.
    fn edges_between_unchecked(&self, src_id: usize, dst_id: usize) -> Vec<&E> {
        self.graph
            .edges_between_unchecked(src_id, dst_id)
            .into_iter()
            .filter(|edge| self.contains_edge(edge.get_id()))
            .collect()
    }

    /// # Arguments
    /// * `src_id`: Id of source vertex.
    /// * `dst_id`: Id of destination vertex.
    /// * `edge_id`: Id of the edge to retrieve.
    ///
    /// # Returns
    /// * `Err`: 
    ///     * If either vertices with `src_id` or `dst_id` is not present in the subgraph.
    ///     * If there is no edge from source to destination with id: `edge_id`.
    /// * `Ok`: Containing reference to edge with id: `edge_id` from `src_id` to `dst_id`.
    fn edge_between(&self, src_id: usize, dst_id: usize, edge_id: usize) -> Result<&E> {
        if !self.contains_vertex(src_id) {
            Err(Error::new_vnf(src_id))?
        } else if !self.contains_vertex(dst_id) {
            Err(Error::new_vnf(dst_id))?
        } else if !self.contains_edge(edge_id) {
            Err(Error::new_enf(edge_id))?
        } else {
            Ok(self.edge_between_unchecked(src_id, dst_id, edge_id))
        }
    }

    /// # Arguments
    /// * `src_id`: Id of source vertex.
    /// * `dst_id`: Id of destination vertex.
    /// * `edge_id`: Id of the edge to retrieve.
    ///
    /// # Returns
    /// Reference to edge with id: `edge_id` from `src_id` to `dst_id`.
    fn edge_between_unchecked(&self, src_id: usize, dst_id: usize, edge_id: usize) -> &E {
        self.graph.edge_between_unchecked(src_id, dst_id, edge_id)
    }

    /// # Note:
    /// Consider using `edge_between` or `edges_from` functions instead of this one.
    /// Because default implementation of this function iterates over all edges to find the edge with specified id.
    /// So:
    /// * if you have info about source of the edge, consider using `edges_from` function instead.
    /// * if you have info about both source and destination of the edge, consider using `edge_between` function instead.
    ///
    /// # Arguments
    /// `edge_id`: Id of the edge to be retrieved.
    ///
    /// # Returns
    /// * `Err`: If there is not edge with id: `edge_id`.
    /// * `Ok`: Containing reference to edge with id: `edge_id`.
    fn edge(&self, edge_id: usize) -> Result<&E> {
        if !self.contains_edge(edge_id) {
            Err(Error::new_enf(edge_id))?
        } else {
            Ok(self.edge_unchecked(edge_id))
        }
    }

    /// # Note:
    /// Consider using `edge_between_unchecked` or `edges_from_unchecked` functions instead of this one.
    /// Because default implementation of this function iterates over all edges to find the edge with specified id.
    /// So:
    /// * if you have info about source of the edge, consider using `edges_from_unchecked` function instead.
    /// * if you have info about both source and destination of the edge, consider using `edge_between_unchecked` function instead.
    ///
    /// # Arguments
    /// `edge_id`: Id of the edge to be retrieved.
    ///
    /// # Returns
    /// Reference to edge with id: `edge_id`.
    fn edge_unchecked(&self, edge_id: usize) -> &E {
        self.graph.edge_unchecked(edge_id)
    }

    /// # Arguments
    /// * `src_id`: Id of the source vertex.
    /// * `dst_id`: Id of the destination vertex.
    ///
    /// # Returns
    /// * `Err`: If either `src_id` or `dst_id` is invalid.
    /// * `Ok`: Containing `true` if there is at least one edge from `src_id` to `dst_id` and `false` otherwise.
    fn has_any_edge(&self, src_id: usize, dst_id: usize) -> Result<bool> {
        if !self.contains_vertex(src_id) {
            Err(Error::new_vnf(src_id))?
        } else if !self.contains_vertex(dst_id) {
            Err(Error::new_vnf(dst_id))?
        } else {
            Ok(self.has_any_edge_unchecked(src_id, dst_id))
        }
    }

    /// # Arguments
    /// * `src_id`: Id of the source vertex.
    /// * `dst_id`: Id of the destination vertex.
    ///
    /// # Returns
    /// `true` if there is at least one edge from `src_id` to `dst_id` and `false` otherwise.
    fn has_any_edge_unchecked(&self, src_id: usize, dst_id: usize) -> bool {
        self.edges
            .iter()
            .find(|(s_id, d_id, _)| *s_id == src_id && *d_id == dst_id)
            .is_some()
    }

    /// # Returns
    /// All edges in the subgraph in the format: (`src_id`, `dst_id`, `edge`).
    fn edges(&self) -> Vec<(usize, usize, &E)> {
        self.graph
            .edges()
            .into_iter()
            .filter(|(src_id, dst_id, edge)| {
                self.contains_vertex(*src_id)
                    && self.contains_vertex(*dst_id)
                    && self.contains_edge(edge.get_id())
            })
            .collect()
    }

    /// Difference between this function and `edges` is that this function treats each edge as a directed edge. \
    /// For example consider graph: a --- b \
    /// If you call `edges` on this graph, you will get: (a, b, edge). \
    /// But if you call `as_directed_edges`, you will get two elements: (a, b, edge) and (b, a, edge). \
    /// It's specifically useful in algorithms that are for directed graphs but can also be applied to undirected graphs if we treat the edges as directed.
    /// One example is [`BellmanFord`](crate::algo::BellmanFord) algorithm.
    ///
    /// # Returns
    /// All edges(as directed edges) in the graph in the format of: (`src_id`, `dst_id`, `edge`).
    fn as_directed_edges(&self) -> Vec<(usize, usize, &E)> {
        if Dir::is_directed() {
            self.edges()
        } else {
            self.edges()
                .into_iter()
                .filter(|(src_id, dst_id, _)| src_id <= dst_id)
                .collect()
        }
    }

    /// # Returns
    /// Number of edges in the graph.
    fn edges_count(&self) -> usize {
        self.edges().len()
    }

    /// # Arguments
    /// `edge_id`: Id of the edge to be found.
    ///
    /// # Returns
    /// * `true`: If edge with id: `edge_id` is present in the subgraph.
    /// * `false`: otherwise.
    fn contains_edge(&self, edge_id: usize) -> bool {
        self.edges
            .iter()
            .find(|(_, _, e_id)| *e_id == edge_id)
            .is_some()
    }
}

impl<'a, W, E, Dir, G> AsFrozenSubgraph<W, E> for Subgraph<'a, W, E, Dir, G>
where
    E: Edge<W>,
    Dir: EdgeDir,
    G: Graph<W, E, Dir> + Edges<W, E> + Neighbors,
{
}

impl<'a, W, E, Dir, G> AsSubgraph<W, E> for Subgraph<'a, W, E, Dir, G>
where
    E: Edge<W>,
    Dir: EdgeDir,
    G: Graph<W, E, Dir> + Vertices + Neighbors + Edges<W, E>,
{
    /// Removes an edge from the subgraph.
    ///
    /// # Arguments
    /// * `src_id`: Id of the source vertex.
    /// * `dst_id`: Id of the destination vertex.
    /// * `edge_id`: Id of the edge from source to destination to be removed.
    ///
    /// # Returns
    /// * `Err`: 
    ///     * If either vertices with `src_id` or `dst_id` does not exist.
    ///     * If there is no edge from source to destination with id: `edge_id`.
    /// * `Ok`:
    fn remove_edge(&mut self, src_id: usize, dst_id: usize, edge_id: usize) -> Result<()> {
        if !self.contains_vertex(src_id) {
            Err(Error::new_vnf(src_id))?
        } else if !self.contains_vertex(dst_id) {
            Err(Error::new_vnf(dst_id))?
        } else if !self.contains_edge(edge_id) {
            Err(Error::new_enf(edge_id))?
        } else {
            Ok(self.remove_edge_unchecked(src_id, dst_id, edge_id))
        }
    }

    /// Removes an edge from the subgraph.
    ///
    /// # Arguments
    /// * `src_id`: Id of the source vertex.
    /// * `dst_id`: Id of the destination vertex.
    /// * `edge_id`: Id of the edge from source to destination to be removed.
    fn remove_edge_unchecked(&mut self, _: usize, _: usize, edge_id: usize) {
        self.edges.retain(|(_, _, e_id)| *e_id != edge_id)
    }

    /// Removes a vertex from the subgraph.
    ///
    /// # Arguments
    /// `vertex_id`: Id of the vertex to be removed.
    ///
    /// # Returns
    /// * `Err`: If vertex with id: `vertex_id` is not present in the subgraph.
    /// * `Ok`:
    fn remove_vertex(&mut self, vertex_id: usize) -> Result<()> {
        if !self.contains_vertex(vertex_id) {
            Err(Error::new_vnf(vertex_id))?
        } else {
            Ok(self.remove_vertex_unchecked(vertex_id))
        }
    }

    /// Removes a vertex from the subgraph.
    ///
    /// # Arguments
    /// `vertex_id`: Id of the vertex to be removed.
    fn remove_vertex_unchecked(&mut self, vertex_id: usize) {
        self.vertex_ids.retain(|v_id| *v_id != vertex_id);

        self.edges
            .retain(|(src_id, dst_id, _)| *src_id != vertex_id && *dst_id != vertex_id);
    }

    /// Adds a vertex from the graph to subgraph.
    ///
    /// # Arguments
    /// `vertex_id`: Id of the vertex to be added.
    ///
    /// # Returns
    /// * `Err`: If graph does not contain vertex with id: `vertex_id`.
    /// * `Ok`:
    fn add_vertex_from_graph(&mut self, vertex_id: usize) -> Result<()> {
        if !self.graph.contains_vertex(vertex_id) {
            Err(Error::new_vnf(vertex_id))?
        } else {
            Ok(self.add_vertex_from_graph_unchecked(vertex_id))
        }
    }

    /// Adds a vertex from the graph to subgraph.
    ///
    /// # Arguments
    /// `vertex_id`: Id of the vertex to be added.
    fn add_vertex_from_graph_unchecked(&mut self, vertex_id: usize) {
        self.vertex_ids.insert(vertex_id);
    }

    /// Adds an edge from the graph to subgraph.
    ///
    /// # Arguments
    /// * `src_id`: Id of the source vertex.
    /// * `dst_id`: Id of the destination vertex.
    /// * `edge_id`: Id of the edge to be added.
    ///
    /// # Returns
    /// * `Err`:
    ///     * If vertex with id: `src_id` does not exist in graph.
    ///     * If vertex with id: `dst_id` dost not exist in graph.
    ///     * If edge with id: `edge_id` does not exist in graph(from src to dst).
    ///     * If edge already exists in the subgraph.
    /// * `Ok`:
    fn add_edge_from_graph(&mut self, src_id: usize, dst_id: usize, edge_id: usize) -> Result<()> {
        if !self.graph.contains_vertex(src_id) {
            Err(Error::new_vnf(src_id))?
        } else if !self.graph.contains_vertex(dst_id) {
            Err(Error::new_vnf(dst_id))?
        } else if !self.graph.contains_edge(edge_id) {
            Err(Error::new_enf(edge_id))?
        } else if self.contains_edge(edge_id) {
            Err(Error::new_eae(edge_id))?
        } else {
            Ok(self.add_edge_from_graph_unchecked(src_id, dst_id, edge_id))
        }
    }

    /// Adds an edge from the graph to subgraph.
    ///
    /// # Arguments
    /// * `src_id`: Id of the source vertex.
    /// * `dst_id`: Id of the destination vertex.
    /// * `edge_id`: Id of the edge to be added.
    fn add_edge_from_graph_unchecked(&mut self, src_id: usize, dst_id: usize, edge_id: usize) {
        self.edges.push((src_id, dst_id, edge_id));

        self.vertex_ids.insert(src_id);
        self.vertex_ids.insert(dst_id);
    }
}
