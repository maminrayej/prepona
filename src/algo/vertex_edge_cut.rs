use std::marker::PhantomData;

use magnitude::Magnitude;

use crate::{
    graph::{Edge, UndirectedEdge},
    provide::{Edges, Graph, IdMap, Vertices},
};

pub struct VertexEdgeCut<'a, W, E: Edge<W>> {
    is_visited: Vec<bool>,
    depth_of: Vec<Magnitude<usize>>,
    low_of: Vec<Magnitude<usize>>,
    parent_of: Vec<Magnitude<usize>>,
    id_map: IdMap,
    cut_vertices: Vec<usize>,
    cut_edges: Vec<(usize, usize, &'a E)>,

    phantom_w: PhantomData<W>,
}

impl<'a, W, E: Edge<W>> VertexEdgeCut<'a, W, E> {
    pub fn init<G>(graph: &G) -> Self
    where
        G: Vertices + Edges<W, E> + Graph<W, E, UndirectedEdge>,
    {
        let vertex_count = graph.vertex_count();

        VertexEdgeCut {
            is_visited: vec![false; vertex_count],
            depth_of: vec![Magnitude::PosInfinite; vertex_count],
            low_of: vec![Magnitude::PosInfinite; vertex_count],
            parent_of: vec![Magnitude::PosInfinite; vertex_count],
            id_map: graph.continuos_id_map(),
            cut_vertices: vec![],
            cut_edges: vec![],

            phantom_w: PhantomData,
        }
    }

    pub fn execute<G>(mut self, graph: &'a G) -> (Vec<usize>, Vec<(usize, usize, &E)>)
    where
        G: Vertices + Edges<W, E> + Graph<W, E, UndirectedEdge>,
    {
        if !self.is_visited.is_empty() {
            self.find_cut_vertices(graph, 0, 0.into());
        }

        (self.cut_vertices, self.cut_edges)
    }

    fn find_cut_vertices<G>(&mut self, graph: &'a G, real_id: usize, depth: Magnitude<usize>)
    where
        G: Vertices + Edges<W, E>,
    {
        let virt_id = self.id_map.virt_id_of(real_id);

        let mut child_count = 0;
        let mut is_vertex_cut = false;
        self.is_visited[virt_id] = true;
        self.depth_of[virt_id] = depth;
        self.low_of[virt_id] = depth;

        for (n_real_id, edge) in graph.edges_from(virt_id) {
            let n_virt_id = self.id_map.virt_id_of(n_real_id);

            if !self.is_visited[n_virt_id] {
                self.parent_of[n_virt_id] = virt_id.into();
                self.find_cut_vertices(graph, n_real_id, depth + 1.into());
                child_count += 1;
                is_vertex_cut = self.low_of[n_virt_id] >= self.depth_of[virt_id];
                if self.low_of[n_virt_id] > self.depth_of[virt_id] {
                    self.cut_edges.push((real_id, n_real_id, edge));
                }
                self.low_of[virt_id] = std::cmp::min(self.low_of[virt_id], self.low_of[n_virt_id]);
            } else if self.parent_of[virt_id] != n_virt_id.into() {
                self.low_of[virt_id] =
                    std::cmp::min(self.low_of[virt_id], self.depth_of[n_virt_id]);
            }
        }

        if (self.parent_of[virt_id].is_finite() && is_vertex_cut)
            || (!self.parent_of[virt_id].is_finite() && child_count > 1)
        {
            self.cut_vertices.push(real_id);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::MatGraph;
    use crate::storage::Mat;

    #[test]
    fn empty_graph() {
        let graph = MatGraph::init(Mat::<usize>::init());

        let (cut_vertices, cut_edges) = VertexEdgeCut::init(&graph).execute(&graph);

        assert!(cut_vertices.is_empty());
        assert!(cut_edges.is_empty());
    }

    #[test]
    fn one_vertex_graph() {
        let mut graph = MatGraph::init(Mat::<usize>::init());
        graph.add_vertex();

        let (cut_vertices, cut_edges) = VertexEdgeCut::init(&graph).execute(&graph);

        assert!(cut_vertices.is_empty());
        assert!(cut_edges.is_empty());
    }

    #[test]
    fn two_vertex_graph() {
        // a --- b
        let mut graph = MatGraph::init(Mat::<usize>::init());
        let a = graph.add_vertex();
        let b = graph.add_vertex();
        let ab = graph.add_edge(a, b, 1.into());

        let (cut_vertices, cut_edges) = VertexEdgeCut::init(&graph).execute(&graph);

        assert!(cut_vertices.is_empty());
        assert_eq!(cut_edges.len(), 1);
        assert!(cut_edges
            .iter()
            .find(|(_, _, edge)| edge.get_id() == ab)
            .is_some());
    }

    #[test]
    fn trivial_graph() {
        // Give:
        //
        //      a --- b
        //      |
        //      c
        let mut graph = MatGraph::init(Mat::<usize>::init());
        let a = graph.add_vertex();
        let b = graph.add_vertex();
        let c = graph.add_vertex();
        let ab = graph.add_edge(a, b, 1.into());
        let ac = graph.add_edge(a, c, 1.into());

        let (cut_vertices, cut_edges) = VertexEdgeCut::init(&graph).execute(&graph);

        assert_eq!(cut_vertices.len(), 1);
        assert!(cut_vertices.contains(&a));
        assert_eq!(cut_edges.len(), 2);
        assert!(vec![ab, ac].into_iter().all(|edge_id| cut_edges
            .iter()
            .find(|(_, _, edge)| edge.get_id() == edge_id)
            .is_some()))
    }

    #[test]
    fn trivial_graph_2() {
        // Given:
        //
        //      a --- b
        //      |       \
        //      |        c
        //      d
        let mut graph = MatGraph::init(Mat::<usize>::init());
        let a = graph.add_vertex();
        let b = graph.add_vertex();
        let c = graph.add_vertex();
        let d = graph.add_vertex();
        let ab = graph.add_edge(a, b, 1.into());
        let bc = graph.add_edge(a, d, 1.into());
        let ad = graph.add_edge(b, c, 1.into());

        let (cut_vertices, cut_edges) = VertexEdgeCut::init(&graph).execute(&graph);

        assert_eq!(cut_vertices.len(), 2);
        assert!(vec![a, b]
            .iter()
            .all(|vertex_id| cut_vertices.contains(vertex_id)));
        assert_eq!(cut_edges.len(), 3);
        assert!(vec![ab, bc, ad].into_iter().all(|edge_id| cut_edges
            .iter()
            .find(|(_, _, edge)| edge.get_id() == edge_id)
            .is_some()))
    }

    #[test]
    fn complex_graph() {
        // Given
        //                                  j
        //                                /   \
        //      a --- b                 i      k ---
        //      |     |                 |           |
        //      c --- d --- e --- f --- h --- m --- l
        //                              |     |
        //                              g     n
        //
        let mut graph = MatGraph::init(Mat::<usize>::init());
        let a = graph.add_vertex();
        let b = graph.add_vertex();
        let c = graph.add_vertex();
        let d = graph.add_vertex();
        let e = graph.add_vertex();
        let f = graph.add_vertex();
        let g = graph.add_vertex();
        let h = graph.add_vertex();
        let i = graph.add_vertex();
        let j = graph.add_vertex();
        let k = graph.add_vertex();
        let l = graph.add_vertex();
        let m = graph.add_vertex();
        let n = graph.add_vertex();
        graph.add_edge(a, b, 1.into());
        graph.add_edge(a, c, 1.into());

        graph.add_edge(b, d, 1.into());

        graph.add_edge(c, d, 1.into());

        let de = graph.add_edge(d, e, 1.into());

        let ef = graph.add_edge(e, f, 1.into());

        let fh = graph.add_edge(f, h, 1.into());

        graph.add_edge(h, i, 1.into());
        let hg = graph.add_edge(h, g, 1.into());
        graph.add_edge(h, m, 1.into());

        graph.add_edge(i, j, 1.into());

        graph.add_edge(j, k, 1.into());

        graph.add_edge(k, l, 1.into());

        graph.add_edge(l, m, 1.into());

        let mn = graph.add_edge(m, n, 1.into());

        let (cut_vertices, cut_edges) = VertexEdgeCut::init(&graph).execute(&graph);

        assert_eq!(cut_vertices.len(), 5);
        assert!(vec![d, e, f, h, m]
            .iter()
            .all(|vertex_id| cut_vertices.contains(vertex_id)));
        assert_eq!(cut_edges.len(), 5);
        assert!(vec![de, ef, fh, hg, mn].into_iter().all(|edge_id| cut_edges
            .iter()
            .find(|(_, _, edge)| edge.get_id() == edge_id)
            .is_some()))
    }

    #[test]
    fn non_bridge_edge_between_two_cut_vertices() {
        // Given:
        //            .-- c --.
        //            |       |
        //      a --- b ----- d --- e
        //
        let mut graph = MatGraph::init(Mat::<usize>::init());
        let a = graph.add_vertex();
        let b = graph.add_vertex();
        let c = graph.add_vertex();
        let d = graph.add_vertex();
        let e = graph.add_vertex();
        let ab = graph.add_edge(a, b, 1.into());
        graph.add_edge(b, c, 1.into());
        graph.add_edge(c, d, 1.into());
        graph.add_edge(b, d, 1.into());
        let de = graph.add_edge(d, e, 1.into());

        let (cut_vertices, cut_edges) = VertexEdgeCut::init(&graph).execute(&graph);

        assert_eq!(cut_vertices.len(), 2);
        assert!(vec![b, d]
            .iter()
            .all(|vertex_id| cut_vertices.contains(vertex_id)));
        assert_eq!(cut_edges.len(), 2);
        assert!(vec![ab, de].into_iter().all(|edge_id| cut_edges
            .iter()
            .find(|(_, _, edge)| edge.get_id() == edge_id)
            .is_some()))
    }
}
