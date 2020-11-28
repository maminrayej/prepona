use std::marker::PhantomData;

use crate::provide::{Edges, Graph, IdMap, Vertices};

use crate::graph::{subgraph::MultiRootSubgraph, Edge, UndirectedEdge};

pub struct ConnectedComponents<'a, W, E: Edge<W>> {
    visited: Vec<bool>,
    edges: Vec<(usize, usize, &'a E)>,
    roots: Vec<usize>,

    phantom_w: PhantomData<W>,
}

impl<'a, W, E: Edge<W>> ConnectedComponents<'a, W, E> {
    pub fn init<G>(graph: &'a G) -> Self
    where
        G: Graph<W, E, UndirectedEdge> + Vertices + Edges<W, E>,
    {
        ConnectedComponents {
            visited: vec![false; graph.vertex_count()],
            edges: vec![],
            roots: vec![],

            phantom_w: PhantomData,
        }
    }

    fn next_start_id(&self) -> Option<usize> {
        self.visited.iter().position(|visited| *visited == false)
    }

    pub fn execute<G>(mut self, graph: &'a G) -> MultiRootSubgraph<'a, W, E, UndirectedEdge, G>
    where
        G: Graph<W, E, UndirectedEdge> + Vertices + Edges<W, E>,
    {
        let id_map = graph.continuos_id_map();

        while let Some(start_id) = self.next_start_id() {
            self.cc_starting_with(graph, &id_map, start_id)
        }

        let mut vertices = self
            .edges
            .iter()
            .flat_map(|(src_id, dst_id, _)| vec![*src_id, *dst_id])
            .collect::<Vec<usize>>();

        // Remove duplicated vertices.
        vertices.sort();
        vertices.dedup();

        MultiRootSubgraph::init(graph, self.edges, vertices, self.roots)
    }

    fn cc_starting_with<G>(&mut self, graph: &'a G, id_map: &IdMap, start_id: usize)
    where
        G: Graph<W, E, UndirectedEdge> + Vertices + Edges<W, E>,
    {
        self.roots.push(start_id);

        let start_virt_id = id_map.virt_id_of(start_id);

        let mut stack = vec![start_virt_id];

        while let Some(virt_id) = stack.pop() {
            self.visited[virt_id] = true;

            let real_id = id_map.real_id_of(virt_id);

            for (n_real_id, edge) in graph.edges_from(real_id) {
                let n_virt_id = id_map.virt_id_of(n_real_id);

                if self.visited[n_virt_id] == false {
                    self.edges.push((real_id, n_real_id, edge));

                    stack.push(n_virt_id);
                }
            }
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

        let ccs = ConnectedComponents::init(&graph).execute(&graph);

        assert_eq!(ccs.roots().len(), 0);
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

        graph.add_edge(a, b, 1.into());
        graph.add_edge(a, c, 1.into());
        graph.add_edge(c, b, 1.into());
        graph.add_edge(b, d, 1.into());
        graph.add_edge(d, e, 1.into());
        graph.add_edge(e, f, 1.into());
        graph.add_edge(f, g, 1.into());

        let ccs = ConnectedComponents::init(&graph).execute(&graph);

        assert_eq!(ccs.roots().len(), 1);
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
        graph.add_vertex();

        graph.add_edge(a, b, 1.into());
        graph.add_edge(a, c, 1.into());
        graph.add_edge(c, b, 1.into());
        graph.add_edge(b, d, 1.into());
        graph.add_edge(e, f, 1.into());

        let ccs = ConnectedComponents::init(&graph).execute(&graph);

        assert_eq!(ccs.roots().len(), 3);
        assert_eq!(ccs.edges_count(), graph.edges_count());
    }

    #[test]
    fn graph_with_no_edge() {
        //      a       b       c
        //      d       e       f
        let mut graph = MatGraph::init(Mat::<usize>::init());
        graph.add_vertex();
        graph.add_vertex();
        graph.add_vertex();
        graph.add_vertex();
        graph.add_vertex();
        graph.add_vertex();

        let ccs = ConnectedComponents::init(&graph).execute(&graph);

        assert_eq!(ccs.roots().len(), 6);
    }
}
