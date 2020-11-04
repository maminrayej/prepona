mod id_map;

pub use id_map::IdMap;

use crate::graph::Edge;

/// Trait to guarantee that graph can provide access to neighbors of a vertex.
pub trait Neighbors {
    /// # Returns:
    /// Id of neighbors of the vertex with `src_id`.
    ///
    /// # Arguments:
    /// * `src_id`: Id of the source vertex.
    fn neighbors(&self, src_id: usize) -> Vec<usize>;
}

/// Trait to guarantee that graph can provide access to vertices in the graph.
pub trait Vertices {
    /// # Returns:
    /// Vector of vertex ids that are present in the graph.
    fn vertices(&self) -> Vec<usize>;

    /// # Returns:
    /// Number of vertices present in the graph.
    ///
    /// # Note:
    /// This function has a default implementation but for better performance its usually better to implement it manually.
    fn vertex_count(&self) -> usize {
        self.vertices().len()
    }

    fn continuos_id_map(&self) -> IdMap {
        let mut id_map = IdMap::init();

        self.vertices().iter().enumerate().for_each(|(virt_id, &real_id)| {
            id_map.put_virt_to_real(virt_id, real_id);
            id_map.put_real_to_virt(real_id, virt_id);
        });

        id_map
    }
}

/// Trait to guarantee that graph can provide access to edges in the graph.
///
/// # Generic Parameters:
/// * `W`: Weight of the edge.
/// * `E`: Edge of the graph.
pub trait Edges<W, E: Edge<W>> {
    /// # Returns:
    /// Vector of edges in the format of (`src_id`, `dst_id`, `edge`).
    fn edges(&self, doubles: bool) -> Vec<(usize, usize, &E)>;

    /// # Returns:
    /// Vector of edges from vertex with `src_id` in the format of (`dst_id`, `edge`).
    ///
    /// # Arguments:
    /// * `src_id`: Id of the source vertex.
    ///
    /// # Note:
    /// This function has a default implementation but for better performance its usually better to implement it manually.
    fn edges_from(&self, src_id: usize) -> Vec<(usize, &E)> {
        // 1. From triplets produced by `edges` function, only keep those that their source vertex id is `src_id`.
        // 2. Map each triplet to a pair by discarding the source vertex id
        self.edges(true)
            .into_iter()
            .filter(|(v1, _, _)| *v1 == src_id)
            .map(|(_, v2, edge)| (v2, edge))
            .collect()
    }

    /// # Returns:
    /// Number of edges in the graph.
    ///
    /// # Note:
    /// This function has a default implementation but for better performance its usually better to implement it manually.
    fn edges_count(&self) -> usize {
        self.edges(false).len()
    }
}

/// Trait to guarantee that graph can provide basic functions needed for graph computation.
///
/// # Generic Parameters:
/// * `W`: Weight of the edge.
/// * `E`: Edge of the graph.
pub trait Graph<W, E: Edge<W>> {
    /// Adds a vertex into the graph.
    ///
    /// # Returns:
    /// Id of the newly inserted vertex.
    fn add_vertex(&mut self) -> usize;

    /// Removes a vertex from the graph.
    ///
    /// # Arguments:
    /// * `vertex_id`: Id of the vertex to be removed.
    fn remove_vertex(&mut self, vertex_id: usize);

    /// Adds an edge from vertex with `src_id` to vertex with `dst_id`.
    ///
    /// # Arguments:
    /// * `src_id`: Id of the vertex at the start of the edge.
    /// * `dst_id`: Id of the vertex at the end of the edge.
    /// * `edge`: Edge between `src_id` and `dst_id`.
    fn add_edge(&mut self, src_id: usize, dst_id: usize, edge: E);

    /// Removes the edge from vertex with `src_id` to vertex with `dst_id`.
    ///
    /// # Arguments:
    /// * `src_id`: Id of the vertex at the start of the edge.
    /// * `dst_id`: Id of the vertex at the end of the edge.
    ///
    /// # Returns:
    /// The removed edge.
    fn remove_edge(&mut self, src_id: usize, dst_id: usize) -> E;

    /// # Returns:
    /// `true`: If edges stored in the graph are directed `false` otherwise.
    fn is_directed(&self) -> bool;

    /// # Returns:
    /// `true`: If edges stored in the graph are undirected `false` otherwise.
    fn is_undirected(&self) -> bool {
        !self.is_directed()
    }
}
