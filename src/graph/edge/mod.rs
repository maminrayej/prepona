pub enum EdgeType {
    Directed,
    Undirected,
}

impl EdgeType {
    pub fn is_directed(&self) -> bool {
        match self {
            EdgeType::Directed => true,
            EdgeType::Undirected => false,
        }
    }

    pub fn is_undirected(&self) -> bool {
        !self.is_directed()
    }
}
