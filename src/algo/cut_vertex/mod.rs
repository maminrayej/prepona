use magnitude::Magnitude;

use crate::{
    graph::{Edge, UndirectedEdge},
    provide::{Graph, IdMap, Neighbors, Vertices},
};

pub struct CutVertex {
    is_visited: Vec<bool>,
    depth_of: Vec<Magnitude<usize>>,
    low_of: Vec<Magnitude<usize>>,
    parent_of: Vec<Magnitude<usize>>,
    id_map: IdMap,
    cut_vertices: Vec<usize>,
}

impl CutVertex {
    pub fn init<W, E, G>(graph: &G) -> Self
    where
        E: Edge<W>,
        G: Vertices + Neighbors + Graph<W, E, UndirectedEdge>,
    {
        let vertex_count = graph.vertex_count();

        CutVertex {
            is_visited: vec![false; vertex_count],
            depth_of: vec![Magnitude::PosInfinite; vertex_count],
            low_of: vec![Magnitude::PosInfinite; vertex_count],
            parent_of: vec![Magnitude::PosInfinite; vertex_count],
            id_map: graph.continuos_id_map(),
            cut_vertices: vec![],
        }
    }

    pub fn execute<W, E, G>(mut self, graph: &G) -> Vec<usize>
    where
        E: Edge<W>,
        G: Vertices + Neighbors + Graph<W, E, UndirectedEdge>,
    {
        if !self.is_visited.is_empty() {
            self.find_cut_vertices(graph, 0, 0.into());
        }

        self.cut_vertices
    }

    fn find_cut_vertices<G>(&mut self, graph: &G, real_id: usize, depth: Magnitude<usize>)
    where
        G: Vertices + Neighbors,
    {
        let virt_id = self.id_map.virt_id_of(real_id);

        let mut child_count = 0;
        let mut is_vertex_cut = false;
        self.is_visited[virt_id] = true;
        self.depth_of[virt_id] = depth;
        self.low_of[virt_id] = depth;

        for n_real_id in graph.neighbors(virt_id) {
            let n_virt_id = self.id_map.virt_id_of(n_real_id);

            if !self.is_visited[n_virt_id] {
                self.parent_of[n_virt_id] = virt_id.into();
                self.find_cut_vertices(graph, n_real_id, depth + 1.into());
                child_count += 1;
                is_vertex_cut = self.low_of[n_virt_id] >= self.depth_of[virt_id];
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

        let cut_vertices = CutVertex::init(&graph).execute(&graph);

        assert!(cut_vertices.is_empty());
    }

    #[test]
    fn one_vertex_graph() {
        let mut graph = MatGraph::init(Mat::<usize>::init());
        graph.add_vertex();

        let cut_vertices = CutVertex::init(&graph).execute(&graph);

        assert!(cut_vertices.is_empty());
    }

    #[test]
    fn two_vertex_graph() {
        // a --- b
        let mut graph = MatGraph::init(Mat::<usize>::init());
        let a = graph.add_vertex();
        let b = graph.add_vertex();
        graph.add_edge(a, b, 1.into());

        let cut_vertices = CutVertex::init(&graph).execute(&graph);

        assert!(cut_vertices.is_empty());
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
        graph.add_edge(a, b, 1.into());
        graph.add_edge(a, c, 1.into());

        let cut_vertices = CutVertex::init(&graph).execute(&graph);

        assert_eq!(cut_vertices.len(), 1);
        assert!(cut_vertices.contains(&a));
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
        graph.add_edge(a, b, 1.into());
        graph.add_edge(a, d, 1.into());
        graph.add_edge(b, c, 1.into());

        let cut_vertices = CutVertex::init(&graph).execute(&graph);

        assert_eq!(cut_vertices.len(), 2);
        assert!(vec![a, b]
            .iter()
            .all(|vertex_id| cut_vertices.contains(vertex_id)));
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

        graph.add_edge(d, e, 1.into());

        graph.add_edge(e, f, 1.into());

        graph.add_edge(f, h, 1.into());
        
        graph.add_edge(h, i, 1.into());
        graph.add_edge(h, g, 1.into());
        graph.add_edge(h, m, 1.into());

        graph.add_edge(i, j, 1.into());

        graph.add_edge(j, k, 1.into());

        graph.add_edge(k, l, 1.into());

        graph.add_edge(l, m, 1.into());

        graph.add_edge(m, n, 1.into());

        let cut_vertices = CutVertex::init(&graph).execute(&graph);

        assert_eq!(cut_vertices.len(), 5);
        assert!(vec![d, e, f, h, m]
            .iter()
            .all(|vertex_id| cut_vertices.contains(vertex_id)));
    }
}
