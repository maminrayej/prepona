pub mod prelude;
pub mod algo;

/// Graphs sit on top of storages defined in [`storage`](crate::storage) module and provide logic about how a storage should be used.
///
/// For example [`SimpleGraph`](crate::graph::SimpleGraph) makes sure that no more than one edge is added between two vertices.
/// It also prevents from adding loops to graph. For more information checkout [`SimpleGraph`](crate::graph::SimpleGraph) documentation.
///
/// Subgraphs are immutable views of graphs. Note that subgraphs themselves are mutable but you can not modify the graph through them.
/// Subgraphs may carry more information than just a view of the graph. For example [ShortestPathSubgraph](crate::graph::subgraph::ShortestPathSubgraph) carries a distance map,
/// in addition to the edges and vertices that participate in the shortest path tree.
/// Subgraphs also implements traits defined in [`provide`](crate::provide) module. 
/// So you can treat them like graphs and all algorithms that are applicable to a graph, is also applicable to a subgraph.
pub mod graph;

/// Collection of traits that each define a set of functionalities exposed by a graph.
///
/// Algorithms only depend on functionalities that are defined in this module and not on specific structures like graphs.
/// This enables us to decouple algorithms from data structures that they receive as input.
/// So you can define your own structure wether it's a graph, subgraph or an augmented graph and run algorithms defined in `algo` module on them.
/// All you have to do is to implement the traits that are needed by the algorithm you want to use.
pub mod provide;

/// Storages are structures that graphs use to store information about vertices and edges.
///
/// There are two storage types that are supported:
/// * Adjacency matrix: Is a matrix used to represent a finite graph. 
///                     The elements of the matrix indicate whether pairs of vertices are adjacent or not in the graph.
///                     For more info read [`AdjMatrix`](crate::storage::AdjMatrix).
/// * Adjacency list:   Is a collection of unordered lists used to represent a finite graph. 
///                     Each list describes the set of neighbors of a vertex in the graph.
///                     For more info read [`AdjList`](crate::storage::AdjList)
///
/// Each storage must implement the [`GraphStorage`](crate::storage::GraphStorage) trait.
/// So You can create your own storage and after implementing the `GraphStorage`, pass it to the graph to use it as backend storage of the graph.
///
/// For a more informed decision regarding to which storage to choose, checkout documentation of each storage and compare their memory usage and time complexity of each operation.
pub mod storage;
