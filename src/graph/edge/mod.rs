pub enum EdgeType {
    Directed,
    Undirected,
}

impl EdgeType {
    /// # Returns:
    /// * `true`: If edge is directed, `false` otherwise.
    pub fn is_directed(&self) -> bool {
        matches!(self, EdgeType::Directed)
    }

    /// # Returns:
    /// * `true`: If edge is undirected, `false` otherwise.
    pub fn is_undirected(&self) -> bool {
        matches!(self, EdgeType::Undirected)
    }
}
