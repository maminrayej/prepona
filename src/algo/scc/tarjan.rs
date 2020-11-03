use magnitude::Magnitude;

use crate::graph::Edge;
use crate::provide;

pub struct TarjanSCC {
    stack: Vec<usize>,
    on_stack: Vec<bool>,
    index_of: Vec<Magnitude<usize>>,
    low_link_of: Vec<Magnitude<usize>>,
    index: usize,
    id_map: provide::IdMap,
    scc: Vec<Vec<usize>>,
}

impl TarjanSCC {
    pub fn init<W, E: Edge<W>, G>(graph: &G) -> Self
    where
        G: provide::Graph<W, E> + provide::Vertices + provide::Neighbors,
    {
        if graph.is_undirected() {
            panic!("Can not run SCC algorithm on undirected graph.")
        }

        let vertex_count = graph.vertex_count();

        TarjanSCC {
            stack: vec![],
            on_stack: vec![false; vertex_count],
            index_of: vec![Magnitude::PosInfinite; vertex_count],
            low_link_of: vec![Magnitude::PosInfinite; vertex_count],
            index: 0,
            scc: vec![],
            id_map: graph.continuos_id_map(),
        }
    }

    pub fn execute<W, E: Edge<W>, G>(mut self, graph: &G) -> Vec<Vec<usize>>
    where
        G: provide::Graph<W, E> + provide::Vertices + provide::Neighbors,
    {
        if graph.is_undirected() {
            panic!("Can not run SCC algorithm on undirected graph.")
        }

        for virt_id in 0..graph.vertex_count() {
            if self.index_of[virt_id].is_pos_infinite() {
                self._execute(graph, virt_id);
            }
        }

        self.scc
    }

    pub fn _execute<W, E: Edge<W>, G>(&mut self, graph: &G, virt_id: usize)
    where
        G: provide::Graph<W, E> + provide::Vertices + provide::Neighbors,
    {
        self.index_of[virt_id] = self.index.into();
        self.low_link_of[virt_id] = self.index.into();

        self.index += 1;

        self.stack.push(virt_id);
        self.on_stack[virt_id] = true;

        let real_id = self.id_map.get_virt_to_real(virt_id).unwrap();

        for dst_real_id in graph.neighbors(real_id) {
            let dst_virt_id = self.id_map.get_real_to_virt(dst_real_id).unwrap();

            if self.index_of[dst_virt_id].is_pos_infinite() {
                self._execute(graph, dst_virt_id);

                self.low_link_of[virt_id] =
                    std::cmp::min(self.low_link_of[virt_id], self.low_link_of[dst_virt_id]);
            } else if self.on_stack[dst_virt_id] {
                self.low_link_of[virt_id] =
                    std::cmp::min(self.low_link_of[virt_id], self.low_link_of[dst_virt_id]);
            }
        }

        if self.low_link_of[virt_id] == self.index_of[virt_id] {
            let mut scc = Vec::<usize>::new();
            loop {
                let w_virt_id = self.stack.pop().unwrap();
                self.on_stack[w_virt_id] = false;

                // add w to new scc
                scc.push(self.id_map.get_virt_to_real(w_virt_id).unwrap());
                if w_virt_id == virt_id {
                    break;
                }
            }

            self.scc.push(scc);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::MatGraph;
    use crate::provide::*;
    use crate::storage::Mat;

    #[test]
    fn tarjan_test() {
        //      a  -->  b  <--  f  -->  g
        //     ^ |      |     __^   /```^
        //     | v      v   /   v```    |
        //      d  -->  c -'--> h  -->  i 
        let mut graph = MatGraph::init(Mat::<usize>::init(true));
        let a = graph.add_vertex();
        let b = graph.add_vertex();
        let c = graph.add_vertex();
        let d = graph.add_vertex();
        let f = graph.add_vertex();
        let g = graph.add_vertex();
        let h = graph.add_vertex();
        let i = graph.add_vertex();

        graph.add_edge(a, d, 1.into());
        graph.add_edge(d, a, 1.into());
        graph.add_edge(a, b, 1.into());
        graph.add_edge(d, c, 1.into());
        graph.add_edge(b, c, 1.into());

        graph.add_edge(f, b, 1.into());
        graph.add_edge(f, h, 1.into());
        graph.add_edge(c, h, 1.into());
        graph.add_edge(c, f, 1.into());
        graph.add_edge(f, g, 1.into());
        graph.add_edge(h, i, 1.into());
        graph.add_edge(g, h, 1.into());
        graph.add_edge(i, g, 1.into());

        let mut tags = std::collections::HashMap::<usize, &'static str>::new();
        tags.insert(a, "a");
        tags.insert(b, "b");
        tags.insert(c, "c");
        tags.insert(d, "d");
        tags.insert(f, "f");
        tags.insert(g, "g");
        tags.insert(h, "h");
        tags.insert(i, "i");
        

        let tarjan = TarjanSCC::init(&graph);

        let sccs = tarjan.execute(&graph);

        for scc in sccs {
            println!("{:?}", scc.iter().map(|v_id| tags.get(v_id).unwrap().to_string()).collect::<String>())
        }
    }
}