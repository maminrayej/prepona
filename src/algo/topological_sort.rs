use crate::algo::{Dfs, DfsListener};
use crate::graph::{DirectedEdge, Edge};
use crate::provide;

/// Finds the topological sort of vertices.
///
/// # Examples
/// ```
/// use prepona::prelude::*;
/// use prepona::storage::DiMat;
/// use prepona::graph::MatGraph;
/// use prepona::algo::TopologicalSort;
///
/// // Given: Graph
/// //
/// //      a  -->  b  -->  c  -->  f
/// //      |        \      |
/// //      |         '-----|
/// //      |               |
/// //      |               v
/// //      '-----> d  -->  e
/// let mut graph = MatGraph::init(DiMat::<usize>::init());
/// let a = graph.add_vertex();
/// let b = graph.add_vertex();
/// let c = graph.add_vertex();
/// let d = graph.add_vertex();
/// let e = graph.add_vertex();
/// let f = graph.add_vertex();
///
/// graph.add_edge(a, b, 1.into());
/// graph.add_edge(a, d, 1.into());
/// graph.add_edge(b, c, 1.into());
/// graph.add_edge(b, e, 1.into());
/// graph.add_edge(d, e, 1.into());
/// graph.add_edge(c, e, 1.into());
/// graph.add_edge(c, f, 1.into());
///
/// let sorted_vertices = TopologicalSort::init().execute(&graph);
///
/// assert_eq!(sorted_vertices.len(), 6);
/// for (src_id, dst_id, _) in graph.edges() {
///     // src must appear before dst in topological sort
///     let src_index = sorted_vertices
///         .iter()
///         .position(|v_id| *v_id == src_id)
///         .unwrap();
///     let dst_index = sorted_vertices
///         .iter()
///         .position(|v_id| *v_id == dst_id)
///         .unwrap();
///
///     assert!(src_index < dst_index)
/// }
/// ```
pub struct TopologicalSort {
    sorted_vertex_ids: Vec<usize>,
}

impl DfsListener for TopologicalSort {
    fn on_black(&mut self, _: &Dfs<Self>, virt_id: usize) {
        self.sorted_vertex_ids.push(virt_id);
    }
}

impl TopologicalSort {
    pub fn init() -> Self {
        TopologicalSort {
            sorted_vertex_ids: vec![],
        }
    }

    /// Finds the topological sort of vertices.
    ///
    /// # Arguments
    /// `graph`: Graph to sort its vertices topologically.
    ///
    /// # Returns
    /// Sorted ids of vertices.
    pub fn execute<W, E: Edge<W>, G>(mut self, graph: &G) -> Vec<usize>
    where
        G: provide::Graph<W, E, DirectedEdge> + provide::Vertices + provide::Neighbors,
    {
        // This algorithm uses dfs to sort the vertices.
        // It stores each vertex the moment dfs visits all its children(vertex color goes black).
        // So a parent will get added after all its children are visited.
        let mut dfs = Dfs::init(graph, &mut self);

        dfs.execute(graph);

        let dfs = dfs;

        let id_map = dfs.dissolve().2;

        // Because a parent is added after its children but in topological order it must be visited first, the order of the vertices added during dfs must be reversed.
        self.sorted_vertex_ids.reverse();

        self.sorted_vertex_ids
            .iter()
            .map(|virt_id| id_map.real_id_of(*virt_id))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::MatGraph;
    use crate::provide::*;
    use crate::storage::DiMat;

    #[test]
    fn empty_graph() {
        let graph = MatGraph::init(DiMat::<usize>::init());

        let sorted_vertices = TopologicalSort::init().execute(&graph);

        assert_eq!(sorted_vertices.len(), 0);
    }

    #[test]
    fn one_vertex_graph() {
        let mut graph = MatGraph::init(DiMat::<usize>::init());
        let _ = graph.add_vertex();

        let sorted_vertices = TopologicalSort::init().execute(&graph);

        assert_eq!(sorted_vertices.len(), 1);
    }

    #[test]
    fn trivial_graph() {
        // Given: Graph
        //
        //      a  -->  b  -->  c  -->  f
        //      |        \      |
        //      |         '-----|
        //      |               |
        //      |               v
        //      '-----> d  -->  e
        let mut graph = MatGraph::init(DiMat::<usize>::init());
        let a = graph.add_vertex();
        let b = graph.add_vertex();
        let c = graph.add_vertex();
        let d = graph.add_vertex();
        let e = graph.add_vertex();
        let f = graph.add_vertex();

        graph.add_edge(a, b, 1.into()).unwrap();
        graph.add_edge(a, d, 1.into()).unwrap();
        graph.add_edge(b, c, 1.into()).unwrap();
        graph.add_edge(b, e, 1.into()).unwrap();
        graph.add_edge(d, e, 1.into()).unwrap();
        graph.add_edge(c, e, 1.into()).unwrap();
        graph.add_edge(c, f, 1.into()).unwrap();

        let sorted_vertices = TopologicalSort::init().execute(&graph);

        assert_eq!(sorted_vertices.len(), 6);
        for (src_id, dst_id, _) in graph.edges() {
            // src must appear before dst in topological sort
            let src_index = sorted_vertices
                .iter()
                .position(|v_id| *v_id == src_id)
                .unwrap();
            let dst_index = sorted_vertices
                .iter()
                .position(|v_id| *v_id == dst_id)
                .unwrap();

            assert!(src_index < dst_index)
        }
    }
}
