mod edge;
mod error;
mod structs;

/// Subgraphs are views of graphs.
///
/// There are multiple types of subgraphs:
/// * [`FrozenSubgraph`](crate::graph::subgraph::AsFrozenSubgraph): Is a subgraph that can not get mutated(An immutable view of the graph).
/// * [`Subgraph`](crate::graph::subgraph::AsSubgraph): Is a subgraph that can get mutated but you can not mutate the graph through this type of subgraph.
/// * [`MutSubgraph`](crate::graph::subgraph::AsMutSubgraph): Is a subgraph that can get mutated and can mutate the graph it's representing.
///
/// Each subgraph must at least implement [`AsFrozenSubgraph`](crate::graph::subgraph::AsFrozenSubgraph) trait.
/// This makes sure that from point of view of algorithms, there is no difference from graph and subgraphs.
///
/// A default [`Subgraph`](crate::graph::subgraph::Subgraph) and [`MutSubgraph`](crate::graph::subgraph::AsMutSubgraph) is defined so you can concentrate on additional functionalities that you want to provide.
/// For example [`ShortestPathSubgraph`](crate::graph::subgraph::ShortestPathSubgraph) is composed of a `Subgraph` and a distance map.
/// So it just forwards every call to `AsSubgraph` functions to the inner `Subgraph`.
pub mod subgraph;

pub use edge::{DefaultEdge, DirectedEdge, Edge, EdgeDir, FlowEdge, UndirectedEdge};
pub use error::{Error, ErrorKind};
pub use structs::{FlowListGraph, FlowMatGraph, ListGraph, MatGraph, SimpleGraph};
