//! Contains data structures and traits necessary to describe, store and retrieve data about edges and vertices.
//!
//! ## Edges
//! Each edge, in the most general sense, can connect multiple sources to multiple destinations[^note].
//! In contrast, in an ordinary graph, an edge connects exactly two vertices.
//! Therefore an [`EdgeDescriptor`] must be flexible enough to model both of these edge types.
//! There are five types of edges that are supported by default:
//! - [`Edge`]
//! - [`Hyperedge`]
//! - [`DirHyperedge`]
//! - [`KUniformHyperedge`]
//! - [`KUniformDirHyperedge`]
//!
//! Any new data structure can be used as an edge throughout the library as long as it implements the [`EdgeDescriptor`] trait.
//!
//! ## Vertices
//! Vertices in a graph can practically be anything. A vertex can contain a number, a text or a whole graph[^graph-in-graph].
//! That's why requirements for a data structure to be a [`VertexDescriptor`] is pretty minimal.
//!
//! ## Storages
//! **NOT IMPLEMENTED YET**
//!
//! [^note]: In even more [`general`] sense an edge can connect to other edges. But, they're not accounted for in our definitions.
//!
//! [^graph-in-graph]: This allows for creation of graphs in which each vertex is a graph itself.
//!
//! [`general`]: https://en.wikipedia.org/wiki/Hypergraph#Further_generalizations
//!
//! [`EdgeDescriptor`]: crate::storage::edge::EdgeDescriptor
//! [`Edge`]: crate::storage::edge::Edge
//! [`Hyperedge`]: crate::storage::edge::Hyperedge
//! [`DirHyperedge`]: crate::storage::edge::DirHyperedge
//! [`KUniformHyperedge`]: crate::storage::edge::KUniformHyperedge
//! [`KUniformDirHyperedge`]: crate::storage::edge::KUniformDirHyperedge
//!
//! [`VertexDescriptor`]: crate::storage::vertex::VertexDescriptor

pub mod edge;
mod errors;
pub mod vertex;

pub use errors::StorageError;
