use crate::graph::Edge;
use crate::provide;
use crate::traversal::Dfs;
use std::cell::RefCell;

pub struct ConnectedComponents {
    ccs: Vec<Vec<usize>>,
}

impl ConnectedComponents {
    pub fn init<G, W, E: Edge<W>>(graph: &G) -> Self
    where
        G: provide::Graph<W, E> + provide::Vertices + provide::Neighbors,
    {
        if graph.is_directed() {
            panic!("Can not execute this algorithm on an undirected graph. Use one of the algorithms in scc module.")
        }

        ConnectedComponents { ccs: vec![] }
    }

    pub fn execute<G, W, E: Edge<W>>(mut self, graph: &G) -> Vec<Vec<usize>>
    where
        G: provide::Graph<W, E> + provide::Vertices + provide::Neighbors,
    {
        let current_component = RefCell::new(vec![]);

        let dfs = Dfs::init(graph);

        dfs.execute(
            graph,
            |_| (),
            |virt_id| {
                let real_id = dfs.get_id_map().get_virt_to_real(virt_id).unwrap();

                current_component.borrow_mut().push(real_id);
            },
            |_| (),
            |_| (),
            || {
                self.ccs.push(current_component.borrow().clone());
                current_component.borrow_mut().clear();
            },
        );

        self.ccs
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::MatGraph;
    use crate::storage::Mat;
    use crate::provide::*;

    #[test]
    fn cc_test() {
        //      a  ---  b   d           g
        //      |      /    |
        //      c ___/      e  --- f
        let mut graph = MatGraph::init(Mat::<usize>::init(false));
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

        graph.add_edge(d, e, 1.into());
        graph.add_edge(e, f, 1.into());

        let mut tags = std::collections::HashMap::<usize, &'static str>::new();
        tags.insert(a, "a");
        tags.insert(b, "b");
        tags.insert(c, "c");
        tags.insert(d, "d");
        tags.insert(e, "e");
        tags.insert(f, "f");
        tags.insert(g, "g");

        let ccs = ConnectedComponents::init(&graph).execute(&graph);

        for cc in ccs {
            println!("{:?}", cc.iter().map(|v_id| tags.get(v_id).unwrap().to_string()).collect::<String>())
        }
    }
}
