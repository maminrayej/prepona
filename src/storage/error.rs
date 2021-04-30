/// Types of errors that may happen when using a graph storage.
pub enum ErrorKind {
    VertexNotFound,
    EdgeNotFound,
    InvalidEdgeId,
    EmptyEdgeList,
}

/// Error type returned by storages in `storage` module.
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

    /// Creates a [`VertexNotFound`](crate::storage::ErrorKind::VertexNotFound) kind of error.
    ///
    /// # Arguments
    /// `vertex_id`: Id of the vertex that has not been found.
    ///
    /// # Returns
    /// `Error` with `VertexNotFound` kind and predefined msg.
    pub fn new_vnf(vertex_id: usize) -> Self {
        Error {
            kind: ErrorKind::VertexNotFound,
            msg: format!("Vertex with id: {} not found", vertex_id),
        }
    }

    /// Creates an [`EdgeNotFound`](crate::storage::ErrorKind::EdgeNotFound) kind of error.
    ///
    /// # Arguments
    /// `edge_id`: Id of the edge that has not been found.
    ///
    /// # Returns
    /// `Error` with `EdgeNotFound` kind and predefined msg.
    pub fn new_enf(edge_id: usize) -> Self {
        Error {
            kind: ErrorKind::EdgeNotFound,
            msg: format!("Edge with id: {} not found", edge_id),
        }
    }

    /// Creates an [`InvalidEdgeId`](crate::storage::ErrorKind::InvalidEdgeId) kind of error.
    ///
    /// # Arguments
    /// * `src_id`: Id of the source vertex.
    /// * `dst_id`: Id of the destination vertex.
    /// * `edge_id`: Id of the edge that was not found from source to destination.
    ///
    /// # Returns
    /// `Error` with `InvalidEdgeId` kind and predefined msg.
    pub fn new_iei(src_id: usize, dst_id: usize, edge_id: usize) -> Self {
        Error {
            kind: ErrorKind::InvalidEdgeId,
            msg: format!(
                "There is no edge with id: {} from vertex with id: {} to vertex with id: {}",
                edge_id, src_id, dst_id
            ),
        }
    }

    /// Creates an [`EmptyEdgeList`](crate::storage::ErrorKind::EmptyEdgeList) kind of error.
    ///
    /// # Arguments
    /// * `src_id`: Id of the source vertex.
    /// * `dst_id`: Id of the destination vertex.
    ///
    /// # Returns
    /// `Error` with `InvalidEdgeId` kind and predefined msg.
    pub fn new_eel(src_id: usize, dst_id: usize) -> Self {
        Error {
            kind: ErrorKind::EmptyEdgeList,
            msg: format!("There is no edge from {} to {}", src_id, dst_id),
        }
    }

    /// # Returns
    /// Cause of the error.
    pub fn msg(&self) -> &String {
        &self.msg
    }

    /// # Returns
    /// Kind of the error.
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
