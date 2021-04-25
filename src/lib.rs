/// Containing algorithms that can get executed on graphs and subgraphs.
///
/// Graphs and subgraphs expose some functionalities defined in the [`provide`](crate::provide) module.
/// One the other end, each algorithm defined in this module require some of the functionalities defined in [`provide`](crate::provide) to be able to get executed.
/// So for one algorithm to be executable on a specific graph or subgraph, it is necessary for the graphs exposed functionalities to match the requirements of the algorithm.
pub mod algo;

/// Re-exports traits and structs that are necessary to accomplish basic tasks with prepona.
pub mod prelude;

/// Graphs sit on top of storages defined in [`storage`](crate::storage) module and provide logic about how a storage should be used.
///
/// For example [`SimpleGraph`](crate::graph::SimpleGraph) makes sure that no more than one edge is added between two vertices.
/// It also prevents from adding loops to graph. For more information checkout [`SimpleGraph`](crate::graph::SimpleGraph) documentation.
pub mod graph;

/// Collection of traits that each define a set of functionalities exposed by a graph.
///
/// Algorithms only depend on functionalities that are defined in this module and not on specific structures like graphs.
/// This enables us to decouple algorithms from data structures that they receive as input.
/// So you can define your own structure wether it's a graph, subgraph or an augmented graph and run algorithms defined in `algo` module on them.
/// All you have to do is to implement the traits that are needed by the algorithm you want to use.
///
/// # Note
/// Functions defined in each trait are abstractions of what is expected from the graphs that implement them.
/// For concrete information about why/when these functions may panic or return `Err`, refer to the specific graph struct that you are using.
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
