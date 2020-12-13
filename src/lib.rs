pub mod prelude;
pub mod algo;
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
