use thiserror::Error;

/// Different kinds of errors that can happen during operations carried out in the storage module.
#[derive(Error, Debug)]
pub enum StorageError {
    /// A vertex is provided that do not match the requirements of some precondition.
    ///
    /// # Arguments
    /// - 0: String representation of the invalid vertex token.
    #[error("Provided vertex token is not valid: {0}")]
    InvalidVertexToken(String),

    /// A vertex was supposed to exist during the operation. But, it was not found.
    ///
    /// # Arguments
    /// - 0: String representation of the vertex token that wasn't found.
    #[error("Vertex with token: {0} is not found")]
    VertexNotFound(String),

    /// An edge was supposed to exist during the operation. But, it was not found.
    ///
    /// # Arguments
    /// - 0: String representation of the edge token that wasn't found.
    #[error("Edge with token: {0} is not found")]
    EdgeNotFound(String),

    /// Number of elements provided for [`KUniformHyperedge`] or [`KUniformDirHyperedge`] was not equal to K.
    ///
    /// # Arguments
    /// - 0: Number of provided elements
    /// - 1: Number of required elements
    ///
    /// [`KUniformHyperedge`]: crate::storage::edge::KUniformHyperedge
    /// [`KUniformDirHyperedge`]: crate::storage::edge::KUniformDirHyperedge
    #[error("Number of provided elements: {0} is not equal to {1}")]
    NotKElement(usize, usize),

    /// [`VertexToken`] provided is not related to a source vertex.
    ///
    /// # Arguments
    /// - 0: String representation of the vertex token.
    ///
    /// [`VertexToken`]: crate::storage::vertex::VertexToken
    #[error("Vertex with token: {0} is not a source")]
    NotSource(String),

    /// [`VertexToken`] provided is not related to a destination vertex.
    ///
    /// # Arguments
    /// - 0: String representation of the vertex token.
    ///
    /// [`VertexToken`]: crate::storage::vertex::VertexToken
    #[error("Vertex with token: {0} is not a destination")]
    NotDestination(String),

    /// [`VertexToken`] provided is a duplicate of an already existing token.
    ///
    /// # Arguments
    /// - 0: String representation of the vertex token.
    ///
    /// [`VertexToken`]: crate::storage::vertex::VertexToken
    #[error("Vertex with token: {0} already exists")]
    VertexAlreadyExists(String),

    /// Request to add connection between a source and a destination is failed because it already exists.
    ///
    /// # Arguments
    /// - 0: String representation of the source vertex.
    /// - 1: String representation of the destination vertex.
    #[error("Connection between vertices with tokens: {0} and {1} already exists")]
    ConnectionAlreadyExists(String, String),
}
