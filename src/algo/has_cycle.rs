use crate::provide;
use crate::traversal::{Color, Dfs, DfsListener};

use magnitude::Magnitude;

pub struct HasCycle<'a, G> {
    parents: Vec<Magnitude<usize>>,
    stack: Vec<usize>,
    cycle: Vec<usize>,
    graph: &'a G,
}

impl<'a, G: provide::Neighbors + provide::Vertices> DfsListener for HasCycle<'a, G> {
    fn on_start(&mut self, _: &Dfs<Self>, virt_id: usize) {
        self.stack.push(virt_id);
    }

    fn on_white(&mut self, dfs: &Dfs<Self>, virt_id: usize) {
        self.parents[virt_id] = (*self.stack.last().unwrap()).into();

        if self.parents[virt_id] != virt_id.into() {
            self.stack.push(virt_id);
        }

        // detect cycle
        let real_id = dfs.get_id_map().get_virt_to_real(virt_id).unwrap();

        if let Some(start_virt_id) = self
            .graph
            .neighbors(real_id)
            .into_iter()
            .map(|real_id| dfs.get_id_map().get_real_to_virt(real_id).unwrap())
            .find(|&n_id| {
                dfs.get_colors()[n_id] != Color::White && self.parents[virt_id] != n_id.into()
            })
        {
            // trace back to vertex with start_virt_id
            let mut parent = virt_id;
            while parent != start_virt_id {
                self.cycle.push(parent);
                parent = self.parents[parent].unwrap();
            }

            self.cycle.push(parent);

            self.cycle.reverse()
        }
    }

    fn on_black(&mut self, _: &Dfs<Self>, _: usize) {
        self.stack.pop();
    }
}

impl<'a, G: provide::Neighbors + provide::Vertices> HasCycle<'a, G> {
    pub fn init(graph: &'a G) -> Self {
        HasCycle {
            parents: vec![Magnitude::PosInfinite; graph.vertex_count()],
            stack: vec![],
            cycle: vec![],
            graph,
        }
    }

    pub fn detect(mut self, graph: &G) -> Vec<usize> {
        let dfs = Dfs::init(graph, &mut self);

        dfs.execute(graph);

        let id_map = dfs.id_map();

        let a = self
            .cycle
            .iter()
            .map(|virt_id| id_map.get_virt_to_real(*virt_id).unwrap())
            .collect();

        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::MatGraph;
    use crate::provide::*;
    use crate::storage::{DiMat, Mat};
    #[test]
    fn directed_simple_cycle() {
        // Give: A directed graph:
        //      a --> b --> c
        //      ^           |
        //      '-----------'
        let mut graph = MatGraph::init(DiMat::<usize>::init());
        let a = graph.add_vertex();
        let b = graph.add_vertex();
        let c = graph.add_vertex();
        graph.add_edge((a, b, 1).into());
        graph.add_edge((b, c, 1).into());
        graph.add_edge((c, a, 1).into());

        let has_cycle = HasCycle::init(&graph);

        let cycle = has_cycle.detect(&graph);

        println!("{:?}", cycle);
    }

    #[test]
    fn undirected_simple_cycle() {
        // Give: An undirected graph:
        //      a -- b -- c
        //      |         |
        //      '---------'
        let mut graph = MatGraph::init(Mat::<usize>::init());
        let a = graph.add_vertex();
        let b = graph.add_vertex();
        let c = graph.add_vertex();
        graph.add_edge((a, b, 1).into());
        graph.add_edge((b, c, 1).into());
        graph.add_edge((c, a, 1).into());

        let has_cycle = HasCycle::init(&graph);

        let cycle = has_cycle.detect(&graph);

        println!("{:?}", cycle);
    }

    #[test]
    fn undirected_single_edge() {
        // Give: An undirected graph:
        //      a -- b
        let mut graph = MatGraph::init(Mat::<usize>::init());
        let a = graph.add_vertex();
        let b = graph.add_vertex();
        graph.add_edge((a, b, 1).into());

        let has_cycle = HasCycle::init(&graph);

        let cycle = has_cycle.detect(&graph);

        assert_eq!(cycle.len(), 0);
    }

    #[test]
    fn directed_single_edge() {
        // Give: An undirected graph:
        //      a --> b
        let mut graph = MatGraph::init(DiMat::<usize>::init());
        let a = graph.add_vertex();
        let b = graph.add_vertex();
        graph.add_edge((a, b, 1).into());

        let has_cycle = HasCycle::init(&graph);

        let cycle = has_cycle.detect(&graph);

        assert_eq!(cycle.len(), 0);
    }
}
