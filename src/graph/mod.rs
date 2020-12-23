mod edge;
mod structs;
mod error;

/// Each subgraph must implement [`AsSubgraph`](crate::graph::subgraph::AsSubgraph) trait.
/// This makes sure that from point of view of algorithms, there is no difference from graph and subgraphs.
///
/// A default [`Subgraph`](crate::graph::subgraph::Subgraph) is defined so you can concentrate on additional functionalities that you want to provide.
/// For example [`ShortestPathSubgraph`](crate::graph::subgraph::ShortestPathSubgraph) is composed of a `Subgraph` and a distance map.
/// So it just forwards every call to `AsSubgraph` functions to the inner `Subgraph`.
pub mod subgraph;

pub use edge::{DefaultEdge, DirectedEdge, Edge, EdgeDir, FlowEdge, UndirectedEdge};
pub use structs::{FlowMatGraph, MatGraph, ListGraph, FlowListGraph, SimpleGraph};
