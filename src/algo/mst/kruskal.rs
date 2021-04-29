use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

use crate::graph::{subgraph::Subgraph, Edge, UndirectedEdge};
use crate::provide;

/// Finds minimum spanning tree using kruskal algorithm.
///
/// # Examples
/// ```
/// use prepona::prelude::*;
/// use prepona::storage::Mat;
/// use prepona::graph::MatGraph;
/// use prepona::algo::Kruskal;
///
/// //  Given: Graph
/// //                5
/// //      f ----------------.
/// //      |                 |
/// //    3 |  1     1     4  |
/// //      a --- b --- d --- e
/// //    3 |   5 |   2 |   1 |
/// //      |     |     |     |
/// //      c ----'-----'-----'
/// let mut graph = MatGraph::init(Mat::<usize>::init());
/// let a = graph.add_vertex();
/// let b = graph.add_vertex();
/// let c = graph.add_vertex();
/// let d = graph.add_vertex();
/// let e = graph.add_vertex();
/// let f = graph.add_vertex();
///
/// let ab = graph.add_edge(a, b, 1.into());
/// graph.add_edge(a, c, 3.into());
/// let af = graph.add_edge(a, f, 3.into());
///
/// graph.add_edge(b, c, 5.into());
/// let bd = graph.add_edge(b, d, 1.into());
///
/// let dc = graph.add_edge(d, c, 2.into());
/// graph.add_edge(d, e, 4.into());
///
/// let ec = graph.add_edge(e, c, 1.into());
/// graph.add_edge(e, f, 5.into());
///
/// let mut tags = std::collections::HashMap::<usize, &'static str>::new();
/// tags.insert(a, "a");
/// tags.insert(b, "b");
/// tags.insert(c, "c");
/// tags.insert(d, "d");
/// tags.insert(e, "e");
/// tags.insert(f, "f");
///
/// let mst = Kruskal::init(&graph).execute(&graph);
///
/// assert_eq!(mst.vertex_count(), 6);
/// assert_eq!(mst.edges_count(), 5);
/// assert!(vec![ab, af, bd, dc, ec].into_iter().all(|edge_id| mst.edge(edge_id).is_ok()))
/// ```
pub struct Kruskal {
    sets: Vec<Rc<RefCell<HashSet<usize>>>>,
}

impl Kruskal {
    /// Initializes the structure.
    pub fn init<G, W: Ord, E: Edge<W>>(graph: &G) -> Self
    where
        G: provide::Vertices + provide::Edges<W, E> + provide::Graph<W, E, UndirectedEdge>,
    {
        let vertex_count = graph.vertex_count();

        let mut sets = vec![];
        sets.resize_with(vertex_count, || Rc::new(RefCell::new(HashSet::new())));

        for virt_id in 0..vertex_count {
            sets[virt_id].borrow_mut().insert(virt_id);
        }

        Kruskal { sets }
    }

    /// Finds minimum spanning tree.
    ///
    /// # Arguments
    /// `graph`: Graph to find its MST.
    ///
    /// # Returns
    /// MST as a subgraph of the original graph(containing vertices and selected edges)
    pub fn execute<'a, G, W: Ord, E: Edge<W>>(
        mut self,
        graph: &'a G,
    ) -> Subgraph<W, E, UndirectedEdge, G>
    where
        G: provide::Edges<W, E>
            + provide::Neighbors
            + provide::Vertices
            + provide::Graph<W, E, UndirectedEdge>,
    {
        let mut mst = Vec::<(usize, usize, &'a E)>::new();

        let id_map = graph.continuos_id_map();

        let mut edges = graph.edges();

        edges.sort_by(|(_, _, e1), (_, _, e2)| e1.get_weight().cmp(e2.get_weight()));

        for (v_real_id, u_real_id, edge) in edges {
            let v_virt_id = id_map.virt_id_of(v_real_id);
            let u_virt_id = id_map.virt_id_of(u_real_id);

            if !self.sets[v_virt_id]
                .borrow()
                .eq(&*self.sets[u_virt_id].borrow())
            {
                mst.push((v_real_id, u_real_id, edge));

                let union_set = self.sets[v_virt_id]
                    .borrow()
                    .union(&*self.sets[u_virt_id].borrow())
                    .copied()
                    .collect::<HashSet<usize>>();

                let sharable_set = Rc::new(RefCell::new(union_set));

                for member in sharable_set.borrow().iter() {
                    self.sets[*member] = sharable_set.clone();
                }
            }
        }

        let vertices = mst
            .iter()
            .flat_map(|(src_id, dst_id, _)| vec![*src_id, *dst_id])
            .collect::<HashSet<usize>>();

        Subgraph::init(graph, mst, vertices)
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

        let mst = Kruskal::init(&graph).execute(&graph);

        assert_eq!(mst.vertex_count(), 0);
    }

    #[test]
    fn trivial_directed_graph() {
        //  Given: Graph
        //                5
        //      f ----------------.
        //      |                 |
        //    3 |  1     1     4  |
        //      a --- b --- d --- e
        //    3 |   5 |   2 |   1 |
        //      |     |     |     |
        //      c ----'-----'-----'
        let mut graph = MatGraph::init(Mat::<usize>::init());
        let a = graph.add_vertex();
        let b = graph.add_vertex();
        let c = graph.add_vertex();
        let d = graph.add_vertex();
        let e = graph.add_vertex();
        let f = graph.add_vertex();

        let ab = graph.add_edge(a, b, 1.into()).unwrap();
        graph.add_edge(a, c, 3.into()).unwrap();
        let af = graph.add_edge(a, f, 3.into()).unwrap();

        graph.add_edge(b, c, 5.into()).unwrap();
        let bd = graph.add_edge(b, d, 1.into()).unwrap();

        let dc = graph.add_edge(d, c, 2.into()).unwrap();
        graph.add_edge(d, e, 4.into()).unwrap();

        let ec = graph.add_edge(e, c, 1.into()).unwrap();
        graph.add_edge(e, f, 5.into()).unwrap();

        let mut tags = std::collections::HashMap::<usize, &'static str>::new();
        tags.insert(a, "a");
        tags.insert(b, "b");
        tags.insert(c, "c");
        tags.insert(d, "d");
        tags.insert(e, "e");
        tags.insert(f, "f");

        let mst = Kruskal::init(&graph).execute(&graph);

        assert_eq!(mst.vertex_count(), 6);
        assert_eq!(mst.edges_count(), 5);
        assert!(vec![ab, af, bd, dc, ec]
            .into_iter()
            .all(|edge_id| mst.edge(edge_id).is_ok()))
    }
}
