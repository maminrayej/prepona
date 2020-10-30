use magnitude::Magnitude;

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
}

/// Trait to guarantee that graph can provide access to edges in the graph.
pub trait Edges<W> {
    /// # Returns:
    /// Vector of edges in the format of (`src_id`, `dst_id`, `weight`).
    fn edges(&self) -> Vec<(usize, usize, Magnitude<W>)>;

    /// # Returns:
    /// Vector of edges from vertex with `src_id` in the format of (`dst_id`, `weight`).
    ///
    /// # Arguments:
    /// * `src_id`: Id of the source vertex.
    ///
    /// # Note:
    /// This function has a default implementation but for better performance its usually better to implement it manually.
    fn edges_from(&self, src_id: usize) -> Vec<(usize, Magnitude<W>)> {
        // 1. From triplets produced by `edges` function, only keep those that their source vertex id is `src_id`.
        // 2. Map each triplet to a pair by discarding the source vertex id
        self.edges()
            .into_iter()
            .filter(|(v1, _, _)| *v1 == src_id)
            .map(|(_, v2, weight)| (v2, weight))
            .collect()
    }

    /// # Returns:
    /// Number of edges in the graph.
    ///
    /// # Note:
    /// This function has a default implementation but for better performance its usually better to implement it manually.
    fn edges_count(&self) -> usize {
        self.edges().len()
    }
}

/// Trait to guarantee that graph can provide basic functions needed for graph computation.
pub trait Graph<W> {
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
    /// * `weight`: Weight of the edge between `src_id` and `dst_id`.
    fn add_edge(&mut self, src_id: usize, dst_id: usize, weight: Magnitude<W>);

    /// Removes the edge from vertex with `src_id` to vertex with `dst_id`.
    ///
    /// # Arguments:
    /// * `src_id`: Id of the vertex at the start of the edge.
    /// * `dst_id`: Id of the vertex at the end of the edge.
    ///
    /// # Returns:
    /// The weight of the removed edge.
    fn remove_edge(&mut self, src_id: usize, dst_id: usize) -> Magnitude<W>;

    /// # Returns:
    /// `true`: If edges stored in the graph are directed `false` otherwise.
    fn is_directed(&self) -> bool;

    /// # Returns:
    /// `true`: If edges stored in the graph are undirected `false` otherwise.
    fn is_undirected(&self) -> bool {
        !self.is_directed()
    }
}
