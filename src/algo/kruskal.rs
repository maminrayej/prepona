use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

use crate::graph::Edge;
use crate::provide;

pub struct Kruskal {
    sets: Vec<Rc<RefCell<HashSet<usize>>>>,
}

impl Kruskal {
    pub fn init<G, W: Ord, E: Edge<W>>(graph: &G) -> Self
    where
        G: provide::Vertices + provide::Edges<W, E>,
    {
        let vertex_count = graph.vertex_count();

        // let sets = vec![; vertex_count];
        let mut sets = vec![];
        sets.resize_with(vertex_count, || Rc::new(RefCell::new(HashSet::new())));

        for virt_id in 0..vertex_count {
            sets[virt_id].borrow_mut().insert(virt_id);
        }

        // sets[0].borrow_mut().insert(1);

        println!("init sets: {:?}", sets);

        Kruskal { sets }
    }

    pub fn execute<'a, G, W: Ord, E: Edge<W>>(mut self, graph: &'a G) -> Vec<(usize, usize)>
    where
        G: provide::Edges<W, E> + provide::Vertices,
    {
        let mut mst = Vec::<(usize, usize)>::new();

        let id_map = graph.continuos_id_map();

        let mut edges = graph.edges(false);

        edges.sort_by(|(_, _, e1), (_, _, e2)| e1.get_weight().cmp(e2.get_weight()));

        for (v_real_id, u_real_id, _) in edges {
            let v_virt_id = id_map.get_real_to_virt(v_real_id).unwrap();
            let u_virt_id = id_map.get_real_to_virt(u_real_id).unwrap();

            if !self.sets[v_virt_id]
                .borrow()
                .eq(&*self.sets[u_virt_id].borrow())
            {
                mst.push((v_real_id, u_real_id));

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

        mst
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::MatGraph;
    use crate::provide::*;
    use crate::storage::Mat;

    #[test]
    fn kruskal_test() {
        let mut graph = MatGraph::init(Mat::<usize>::init(false));
        let a = graph.add_vertex();
        let b = graph.add_vertex();
        let c = graph.add_vertex();
        let d = graph.add_vertex();
        let e = graph.add_vertex();
        let f = graph.add_vertex();

        graph.add_edge(a, b, 1.into());
        graph.add_edge(a, c, 3.into());
        graph.add_edge(a, f, 3.into());

        graph.add_edge(b, c, 5.into());
        graph.add_edge(b, d, 1.into());

        graph.add_edge(d, c, 2.into());
        graph.add_edge(d, e, 4.into());

        graph.add_edge(e, c, 1.into());
        graph.add_edge(e, f, 5.into());

        let mut tags = std::collections::HashMap::<usize, &'static str>::new();
        tags.insert(a, "a");
        tags.insert(b, "b");
        tags.insert(c, "c");
        tags.insert(d, "d");
        tags.insert(e, "e");
        tags.insert(f, "f");

        let kurskal = Kruskal::init(&graph);

        println!(
            "{:?}",
            kurskal
                .execute(&graph)
                .into_iter()
                .map(|(v1, v2)| (tags.get(&v1).unwrap(), tags.get(&v2).unwrap()))
                .collect::<Vec<(&&str, &&str)>>()
        )
    }
}
