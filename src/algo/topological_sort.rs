use crate::graph::{DirectedEdge, Edge};
use crate::provide;
use crate::algo::{Dfs, DfsListener};

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

    pub fn execute<W, E: Edge<W>, G>(mut self, graph: &G) -> Vec<usize>
    where
        G: provide::Graph<W, E, DirectedEdge> + provide::Vertices + provide::Neighbors,
    {
        let dfs = Dfs::init(graph, &mut self);

        dfs.execute(graph);

        let id_map = dfs.id_map();

        self.sorted_vertex_ids.reverse();

        self.sorted_vertex_ids
            .into_iter()
            .map(|virt_id| id_map.get_virt_to_real(virt_id).unwrap())
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
    fn topological_sort_test() {
        // a  -->  b.
        let mut graph = MatGraph::init(DiMat::<usize>::init());
        let a = graph.add_vertex();
        let b = graph.add_vertex();
        let c = graph.add_vertex();
        graph.add_edge((a, b, 1).into());
        graph.add_edge((a, c, 1).into());

        let sorted_ids = TopologicalSort::init().execute(&graph);

        println!("{:?}", sorted_ids);
    }
}
