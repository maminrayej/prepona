mod id_map;

use anyhow::Result;
pub use id_map::IdMap;

use crate::graph::{Edge, EdgeDir};

/// Provides access to neighbors of an arbitrary vertex.
pub trait Neighbors {
    /// # Arguments:
    /// `src_id`: Id of the source vertex.
    ///
    /// # Returns
    /// * `Err`
    /// * `Ok`: Containing Id of vertices accessible from source vertex using one edge.
    fn neighbors(&self, src_id: usize) -> Result<Vec<usize>>;

    /// # Arguments:
    /// `src_id`: Id of the source vertex.
    ///
    /// # Returns
    /// Id of vertices accessible from source vertex using one edge.
    fn neighbors_unchecked(&self, src_id: usize) -> Vec<usize>;
}

/// Provides access to vertices of the graph.
pub trait Vertices {
    /// # Returns
    /// Id of vertices that are present in the graph.
    fn vertices(&self) -> Vec<usize>;

    /// # Returns
    /// Number of vertices in the graph.
    fn vertex_count(&self) -> usize {
        self.vertices().len()
    }

    /// In many algorithms assuming graph vertices have a continuos series of ids makes implementing the algorithms easier.
    /// So this function maps potentially scattered vertex ids into a continuos one.
    /// In this mapping, scattered ids are real and continuos ones are virtual.
    ///
    /// For example: \
    /// if vertex ids are [1, 3, 4], they will be mapped to [0, 1, 2] so you can store information about vertices in a vector and use the new vertex ids as index.
    ///
    /// # Returns
    /// The two-way mapping between scattered and continuos ids.
    fn continuos_id_map(&self) -> IdMap {
        let mut id_map = IdMap::init(self.vertex_count());

        // Map each vertex id to its index in vector returned by vertices.
        self.vertices()
            .iter()
            .enumerate()
            .for_each(|(virt_id, &real_id)| {
                id_map.put_virt_to_real(virt_id, real_id);
                id_map.put_real_to_virt(real_id, virt_id);
            });

        id_map
    }

    /// # Arguments
    /// `vertex_id`: Id of the vertex.
    ///
    /// # Returns
    /// * `true`: if graph contains the vertex with specified id.
    /// * `false`: otherwise.
    fn contains_vertex(&self, vertex_id: usize) -> bool;
}

/// Provides access to edges of the graph.
pub trait Edges<W, E: Edge<W>> {
    /// # Arguments
    /// `src_id`: Id of the source vertex.
    ///
    /// # Returns
    /// * `Err`
    /// * `Ok`: Containing all the outgoing edges from the source vertex in the format of: (`dst_id`, `edge`)
    fn edges_from(&self, src_id: usize) -> Result<Vec<(usize, &E)>>;

    /// # Arguments
    /// `src_id`: Id of the source vertex.
    ///
    /// # Returns
    /// * All edges from the source vertex in the format of: (`dst_id`, `edge`)
    fn edges_from_unchecked(&self, src_id: usize) -> Vec<(usize, &E)>;

    /// # Arguments
    /// * `src_id`: Id of source vertex.
    /// * `dst_id`: Id of destination vertex.
    ///
    /// # Returns
    /// * `Err`:
    /// * `Ok`: Containing edges from source vertex to destination vertex.
    fn edges_between(&self, src_id: usize, dst_id: usize) -> Result<Vec<&E>>;

    /// # Arguments
    /// * `src_id`: Id of source vertex.
    /// * `dst_id`: Id of destination vertex.
    ///
    /// # Returns
    /// Edges from source vertex to destination vertex.
    fn edges_between_unchecked(&self, src_id: usize, dst_id: usize) -> Vec<&E>;

    /// # Arguments
    /// * `src_id`: Id of source vertex.
    /// * `dst_id`: Id of destination vertex.
    /// * `edge_id`: Id of the edge to retrieve.
    ///
    /// # Returns
    /// * `Err`:
    /// * `Ok`: Containing reference to edge with id: `edge_id` from `src_id` to `dst_id`.
    fn edge_between(&self, src_id: usize, dst_id: usize, edge_id: usize) -> Result<&E>;

    /// # Arguments
    /// * `src_id`: Id of source vertex.
    /// * `dst_id`: Id of destination vertex.
    /// * `edge_id`: Id of the edge to retrieve.
    ///
    /// # Returns
    /// Reference to edge with id: `edge_id` from `src_id` to `dst_id`.
    fn edge_between_unchecked(&self, src_id: usize, dst_id: usize, edge_id: usize) -> &E;

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
    /// * `Err`:
    /// * `Ok`: Containing reference to edge with id: `edge_id`.
    fn edge(&self, edge_id: usize) -> Result<&E>;

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
    fn edge_unchecked(&self, edge_id: usize) -> &E;

    /// # Arguments
    /// * `src_id`: Id of the source vertex.
    /// * `dst_id`: Id of the destination vertex.
    ///
    /// # Returns
    /// * `Err`:
    /// * `Ok`: Containing `true` if there is at least one edge from `src_id` to `dst_id` and `false` otherwise.
    fn has_any_edge(&self, src_id: usize, dst_id: usize) -> Result<bool>;

    /// # Arguments
    /// * `src_id`: Id of the source vertex.
    /// * `dst_id`: Id of the destination vertex.
    ///
    /// # Returns
    /// `true` if there is at least one edge from `src_id` to `dst_id` and `false` otherwise.
    fn has_any_edge_unchecked(&self, src_id: usize, dst_id: usize) -> bool;

    /// # Returns
    /// All edges in the graph in the format: (`src_id`, `dst_id`, `edge`).
    fn edges(&self) -> Vec<(usize, usize, &E)>;

    /// Difference between this function and `edges` is that this function treats each edge as a directed edge. \
    /// For example consider graph: a --- b \
    /// If you call `edges` on this graph, you will get: (a, b, edge). \
    /// But if you call `as_directed_edges`, you will get two elements: (a, b, edge) and (b, a, edge). \
    /// It's specifically useful in algorithms that are for directed graphs but can also be applied to undirected graphs if we treat the edges as directed.
    /// One example is [`BellmanFord`](crate::algo::BellmanFord) algorithm.
    ///
    /// # Returns
    /// All edges(as directed edges) in the graph in the format of: (`src_id`, `dst_id`, `edge`).
    fn as_directed_edges(&self) -> Vec<(usize, usize, &E)>;

    /// # Returns
    /// Number of edges in the graph.
    fn edges_count(&self) -> usize;

    /// # Arguments
    /// `edge_id`: Id of the edge.
    ///
    /// # Returns
    /// * `true`: if graph contains the edge with specified id.
    /// * `false`: otherwise.
    fn contains_edge(&self, edge_id: usize) -> bool;
}

/// Provides basic functionalities to store graph information.
pub trait Graph<W, E: Edge<W>, Ty: EdgeDir> {
    /// Adds a vertex to the graph.
    ///
    /// # Returns
    /// Unique id of the newly added vertex.
    fn add_vertex(&mut self) -> usize;

    /// Removes the vertex with id: `vertex_id` from graph.
    ///
    /// # Arguments
    /// `vertex_id`: Id of the vertex to be removed.
    fn remove_vertex(&mut self, vertex_id: usize) -> Result<()>;

    /// Removes the vertex with id: `vertex_id` from graph.
    ///
    /// # Arguments
    /// `vertex_id`: Id of the vertex to be removed.
    fn remove_vertex_unchecked(&mut self, vertex_id: usize);

    /// Adds `edge` from vertex with id `src_id`: to vertex with id: `dst_id`.
    ///
    /// # Arguments
    /// * `src_id`: Id of the source vertex.
    /// * `dst_id`: Id of the destination vertex.
    /// * `edge`: Edge to be added from source to destination.
    ///
    /// # Returns
    /// * `Err`:
    /// * `Ok`: Containing unique id of the newly added edge.
    fn add_edge(&mut self, src_id: usize, dst_id: usize, edge: E) -> Result<usize>;

    /// Adds `edge` from vertex with id `src_id`: to vertex with id: `dst_id`.
    ///
    /// # Arguments
    /// * `src_id`: Id of the source vertex.
    /// * `dst_id`: Id of the destination vertex.
    /// * `edge`: Edge to be added from source to destination.
    ///
    /// # Returns
    /// Unique id of the newly added edge.
    fn add_edge_unchecked(&mut self, src_id: usize, dst_id: usize, edge: E) -> usize;

    /// Replaces the edge with id: `edge_id` with `edge`.
    ///
    /// # Arguments
    /// * `src_id`: Id of source vertex.
    /// * `dst_id`: Id of destination vertex.
    /// * `edge_id`: Id of the to be updated edge.
    /// * `edge`: New edge to replace the old one.
    fn update_edge(&mut self, src_id: usize, dst_id: usize, edge_id: usize, edge: E) -> Result<()>;

    /// Replaces the edge with id: `edge_id` with `edge`.
    ///
    /// # Arguments
    /// * `src_id`: Id of source vertex.
    /// * `dst_id`: Id of destination vertex.
    /// * `edge_id`: Id of the to be updated edge.
    /// * `edge`: New edge to replace the old one.
    fn update_edge_unchecked(&mut self, src_id: usize, dst_id: usize, edge_id: usize, edge: E);

    /// Removes the edge with id: `edge_id`.
    ///
    /// # Arguments
    /// * `src_id`: Id of source vertex.
    /// * `dst_id`: Id of destination vertex.
    /// * `edge_id`: Id of edge to be removed.
    ///
    /// # Returns
    /// * `Err`:
    /// * `Ok`: Containing removed edge.
    fn remove_edge(&mut self, src_id: usize, dst_id: usize, edge_id: usize) -> Result<E>;

    /// Removes the edge with id: `edge_id`.
    ///
    /// # Arguments
    /// * `src_id`: Id of source vertex.
    /// * `dst_id`: Id of destination vertex.
    /// * `edge_id`: Id of edge to be removed.
    ///
    /// # Returns
    /// Removed edge.
    fn remove_edge_unchecked(&mut self, src_id: usize, dst_id: usize, edge_id: usize) -> E;
}
