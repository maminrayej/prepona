/// Types of errors that may happen when using a graph or subgraph.
pub enum ErrorKind {
    Loop,
    MultiEdge,
    VertexNotFound,
    EdgeNotFound,
    EdgeAlreadyExists,
    RootAlreadyExists,
}

/// Error type returns in [`graph`](crate::graph) module.
pub struct Error {
    kind: ErrorKind,
    msg: String,
}

impl Error {
    /// # Arguments
    /// * `kind`: Specifies what kind of error is being created.
    /// * `msg`: Cause of the error.
    ///
    /// # Returns
    /// Constructed `Error`.
    pub fn new(kind: ErrorKind, msg: String) -> Self {
        Error { kind, msg }
    }

    /// Creates a new [`Loop`](crate::graph::ErrorKind::Loop) kind of error.
    ///
    /// # Arguments
    /// * `vertex_id`: Id of the vertex that loop tried to get created on.
    ///
    /// # Returns
    /// `Error` with `Loop` kind and predefined message.
    pub fn new_l(vertex_id: usize) -> Self {
        Error {
            kind: ErrorKind::Loop,
            msg: format!("Can not add edge from vertex: {} to itself", vertex_id),
        }
    }

    /// Creates a new [`MultiEdge`](crate::graph::ErrorKind::MultiEdge) kind of error.
    /// This error is thrown when there is an attempt to add more than one edge from a source to a destination vertex.
    ///
    /// # Arguments
    /// * `src_id`: Id of the source vertex.
    /// * `dst_id`: Id of the destination vertex.
    ///
    /// # Returns
    /// `Error` with `MultiEdge` kind and predefined message.
    pub fn new_me(src_id: usize, dst_id: usize) -> Self {
        Error {
            kind: ErrorKind::MultiEdge,
            msg: format!("There is already an edge from {} to {}", src_id, dst_id),
        }
    }

    /// Creates a new [`VertexNotFound`](crate::graph::ErrorKind::VertexNotFound) kind of error.
    ///
    /// # Arguments
    /// `vertex_id`: If of the vertex that could not be found.
    ///
    /// # Returns
    /// `Error` with `VertexNotFound` kind and predefined message.
    pub fn new_vnf(vertex_id: usize) -> Self {
        Error {
            kind: ErrorKind::VertexNotFound,
            msg: format!("Vertex with id: {} does not exist", vertex_id),
        }
    }

    /// Creates a new [`EdgeNotFound`](crate::graph::ErrorKind::EdgeNotFound) kind of error.
    ///
    /// # Arguments
    /// `edge_id`: If of the.
    ///
    /// # Returns
    /// `Error` with `EdgeNotFound` kind and predefined message.
    pub fn new_enf(edge_id: usize) -> Self {
        Error {
            kind: ErrorKind::EdgeNotFound,
            msg: format!("Edge with id: {} does not exist", edge_id),
        }
    }

    /// Creates a new [`EdgeAlreadyExists`](crate::graph::ErrorKind::EdgeAlreadyExists) kind of error.
    /// This error is thrown when there is an attempt to add an edge to a graph with an id that already exists.
    ///
    /// # Arguments
    /// `edge_id`: Id of the edge that already exists in the graph.
    ///
    /// # Returns
    /// `Error` with `EdgeAlreadyExists` kind and predefined message.
    pub fn new_eae(edge_id: usize) -> Self {
        Error {
            kind: ErrorKind::EdgeAlreadyExists,
            msg: format!("Edge with id: {} already exists", edge_id),
        }
    }

    /// Creates a new [`RootAlreadyExists`](crate::graph::ErrorKind::RootAlreadyExists) kind of error.
    /// This error is thrown in [`MultiRootedSubgraph`](crate::graph::subgraph::MultiRootSubgraph) when there is an attempt to add a vertex to roots that already is a root.
    ///
    /// # Arguments
    /// `vertex_id`: Id of the vertex to be added as root.
    ///
    /// # Returns
    /// `Error` with `RootAlreadyExists` kind and predefined message.
    pub fn new_rae(vertex_id: usize) -> Self {
        Error {
            kind: ErrorKind::RootAlreadyExists,
            msg: format!("Vertex with id: {} is already a root", vertex_id),
        }
    }

    /// # Returns
    /// Message inside of the error.
    pub fn msg(&self) -> &str {
        self.msg.as_str()
    }

    /// # Returns
    /// What kind the error is.
    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.msg())
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.msg())
    }
}

impl std::error::Error for Error {}
