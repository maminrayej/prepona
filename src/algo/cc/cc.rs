use crate::algo::{Dfs, DfsListener};
use crate::graph::{Edge, UndirectedEdge};
use crate::provide;

/// Finds connected components of an undirected graph.
pub struct ConnectedComponents {
    current_component: Vec<usize>,
    ccs: Vec<Vec<usize>>,
}

impl DfsListener for ConnectedComponents {
    fn on_white(&mut self, dfs: &Dfs<Self>, virt_id: usize) {
        let real_id = dfs.get_id_map().real_id_of(virt_id);

        self.current_component.push(real_id);
    }

    fn on_finish(&mut self, _: &Dfs<Self>) {
        self.ccs.push(self.current_component.clone());
        self.current_component.clear();
    }
}

impl ConnectedComponents {
    /// Initializes the structure.
    pub fn init<G, W, E: Edge<W>>(_: &G) -> Self
    where
        G: provide::Graph<W, E, UndirectedEdge> + provide::Vertices + provide::Neighbors,
    {
        ConnectedComponents {
            ccs: vec![],
            current_component: vec![],
        }
    }

    /// Finds connected components of an undirected graph.
    ///
    /// # Arguments
    /// `graph`: Graph to search for its connected components.
    ///
    /// # Returns
    /// Connected components of the graph. \
    /// Returned value will be vector of vectors. Each vector contains ids of vertices that are in a component.
    pub fn execute<G, W, E: Edge<W>>(mut self, graph: &G) -> Vec<Vec<usize>>
    where
        G: provide::Graph<W, E, UndirectedEdge> + provide::Vertices + provide::Neighbors,
    {
        let mut dfs = Dfs::init(graph, &mut self);

        dfs.execute(graph);

        self.ccs
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::MatGraph;
    use crate::provide::*;
    use crate::storage::Mat;

    #[test]
    fn empty_graph() {
        let graph = MatGraph::init(Mat::<usize>::init());

        let ccs = ConnectedComponents::init(&graph).execute(&graph);

        assert_eq!(ccs.len(), 0);
    }

    #[test]
    fn graph_with_one_component() {
        //      a  ---  b  ---  d            g
        //      |      /        |            |
        //      c ___/          '---  e  --- f
        let mut graph = MatGraph::init(Mat::<usize>::init());
        let a = graph.add_vertex();
        let b = graph.add_vertex();
        let c = graph.add_vertex();
        let d = graph.add_vertex();
        let e = graph.add_vertex();
        let f = graph.add_vertex();
        let g = graph.add_vertex();

        graph.add_edge_unchecked(a, b, 1.into());
        graph.add_edge_unchecked(a, c, 1.into());
        graph.add_edge_unchecked(c, b, 1.into());
        graph.add_edge_unchecked(b, d, 1.into());
        graph.add_edge_unchecked(d, e, 1.into());
        graph.add_edge_unchecked(e, f, 1.into());
        graph.add_edge_unchecked(f, g, 1.into());

        let ccs = ConnectedComponents::init(&graph).execute(&graph);

        assert_eq!(ccs.len(), 1);
        assert_eq!(ccs[0].len(), 7);
        assert!(vec![a, b, c, d, e, f, g]
            .iter()
            .all(|v_id| ccs[0].contains(v_id)));
    }

    #[test]
    fn trivial_graph() {
        //      a  ---  b  ---  d               g
        //      |      /
        //      c ___/              e  --- f
        let mut graph = MatGraph::init(Mat::<usize>::init());
        let a = graph.add_vertex();
        let b = graph.add_vertex();
        let c = graph.add_vertex();
        let d = graph.add_vertex();
        let e = graph.add_vertex();
        let f = graph.add_vertex();
        let g = graph.add_vertex();

        graph.add_edge_unchecked(a, b, 1.into());
        graph.add_edge_unchecked(a, c, 1.into());
        graph.add_edge_unchecked(c, b, 1.into());
        graph.add_edge_unchecked(b, d, 1.into());
        graph.add_edge_unchecked(e, f, 1.into());

        let ccs = ConnectedComponents::init(&graph).execute(&graph);

        for cc in ccs {
            match cc.len() {
                1 => assert!(cc.contains(&g)),
                2 => assert!(vec![e, f].iter().all(|v_id| cc.contains(v_id))),
                4 => assert!(vec![a, b, c, d].iter().all(|v_id| cc.contains(v_id))),
                _ => panic!("Unknown component: {:?}", cc),
            }
        }
    }

    #[test]
    fn graph_with_no_edge() {
        //      a       b       c
        //      d       e       f
        let mut graph = MatGraph::init(Mat::<usize>::init());
        let a = graph.add_vertex();
        let b = graph.add_vertex();
        let c = graph.add_vertex();
        let d = graph.add_vertex();
        let e = graph.add_vertex();
        let f = graph.add_vertex();

        let ccs = ConnectedComponents::init(&graph).execute(&graph);

        assert_eq!(ccs.len(), 6);
        for cc in &ccs {
            assert_eq!(cc.len(), 1)
        }
        assert_eq!(ccs.concat(), [a, b, c, d, e, f]);
    }
}
