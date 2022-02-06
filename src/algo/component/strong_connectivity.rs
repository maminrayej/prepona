use anyhow::Result;

use crate::algo::errors::AlgoError;
use crate::algo::traversal::{ControlFlow, EdgeType, Event, DFS};
use crate::common::RealID;
use crate::provide::{Storage, Vertices};
use crate::storage::edge::Directed;

pub fn strongly_connected_components<G>(graph: &G) -> Vec<Vec<usize>>
where
    G: Storage<Dir = Directed> + Vertices,
{
    let mut id = 1;
    let mut id_of = vec![0; graph.vertex_count()];
    let mut low_id = vec![0; graph.vertex_count()];
    let mut stack = vec![];
    let mut sccs = vec![];

    DFS::init(graph).execute(|event| {
        match event {
            Event::Discover(state, v_rid) => {
                let v_vid = state.id_map[v_rid];

                id_of[v_vid.inner()] = id;
                low_id[v_vid.inner()] = id;

                id += 1;
                stack.push(v_rid);
            }
            Event::Finish(state, v_rid) => {
                let v_vid = state.id_map[v_rid];
                let v_low_id = low_id[v_vid.inner()];
                let v_id = id_of[v_vid.inner()];

                if v_id == v_low_id {
                    let mut cc = vec![];
                    while let Some(rid) = stack.pop() {
                        cc.push(rid.inner());

                        if rid == v_rid {
                            break;
                        }
                    }

                    sccs.push(cc);
                } else {
                    if let Some(pid) = state.parent.get(&v_rid.inner()) {
                        let p_vid = state.id_map[RealID::from(*pid)];
                        let p_low_id = low_id[p_vid.inner()];

                        low_id[p_vid.inner()] = std::cmp::min(v_low_id, p_low_id);
                    }
                }
            }
            Event::VisitEdge(state, edge_type) => match edge_type {
                EdgeType::BackEdge(s_rid, d_rid) => {
                    let s_vid = state.id_map[s_rid];
                    let d_vid = state.id_map[d_rid];

                    let s_low_id = low_id[s_vid.inner()];
                    let d_low_id = low_id[d_vid.inner()];

                    low_id[s_vid.inner()] = std::cmp::min(s_low_id, d_low_id);
                }
                _ => {}
            },
            _ => {}
        }

        ControlFlow::Continue
    });

    sccs
}

pub fn number_strongly_connected_components<G>(graph: &G) -> usize
where
    G: Storage<Dir = Directed> + Vertices,
{
    strongly_connected_components(graph).len()
}

pub fn is_strongly_connected<G>(graph: &G) -> Result<bool>
where
    G: Storage<Dir = Directed> + Vertices,
{
    if graph.vertex_count() == 0 {
        Err(
            AlgoError::UndefinedConcept("Connectivity is undefined for null graph".to_string())
                .into(),
        )
    } else {
        Ok(number_strongly_connected_components(graph) == 1)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::algo::component::{
        is_strongly_connected, number_strongly_connected_components, strongly_connected_components,
    };
    use crate::gen::{CompleteGraphGenerator, Generator, NullGraphGenerator};
    use crate::provide::{MutEdges, MutVertices};
    use crate::storage::edge::Directed;
    use crate::storage::AdjMap;
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn prop_strong_connectivity(generator: NullGraphGenerator) {
        let mut graph: AdjMap<(), (), Directed> = generator.generate();

        CompleteGraphGenerator::add_component_to(&mut graph, 3);
        CompleteGraphGenerator::add_component_to(&mut graph, 4);
        CompleteGraphGenerator::add_component_to(&mut graph, 5);
        CompleteGraphGenerator::add_component_to(&mut graph, 6);

        let ccs = strongly_connected_components(&graph);
        let ccs_count = number_strongly_connected_components(&graph);

        assert_eq!(ccs.len(), 4);
        assert_eq!(ccs.into_iter().map(|cc| cc.len()).sum::<usize>(), 18);
        assert_eq!(ccs_count, 4);
        assert!(!is_strongly_connected(&graph).unwrap())
    }

    #[test]
    fn strong_connectivity_specific() {
        let mut graph: AdjMap<(), (), Directed> = AdjMap::init();

        let a = graph.add_vertex(());
        let b = graph.add_vertex(());
        let c = graph.add_vertex(());
        let d = graph.add_vertex(());
        let e = graph.add_vertex(());
        let f = graph.add_vertex(());
        let g = graph.add_vertex(());
        let h = graph.add_vertex(());
        let i = graph.add_vertex(());

        //              .-------------------.
        //              |           .---.   |
        //              |           |   |   |
        // a <-> b <--- f <-- g <-- i<--'   |
        // ^     |      |     ^     |       |
        // |     |      |     |     |       |
        // v     v      v     |     |       |
        // c --> d <--> e <-- h <---'-------'
        graph.add_edge(a, b, ());
        graph.add_edge(a, c, ());

        graph.add_edge(b, a, ());
        graph.add_edge(b, d, ());

        graph.add_edge(c, a, ());
        graph.add_edge(c, d, ());

        graph.add_edge(d, e, ());

        graph.add_edge(e, d, ());

        graph.add_edge(f, b, ());
        graph.add_edge(f, e, ());
        graph.add_edge(f, h, ());

        graph.add_edge(g, f, ());

        graph.add_edge(h, g, ());
        graph.add_edge(h, e, ());

        graph.add_edge(i, i, ());
        graph.add_edge(i, g, ());
        graph.add_edge(i, h, ());

        let ccs = strongly_connected_components(&graph);
        assert_eq!(ccs.len(), 4);
        assert_eq!(
            HashSet::from([1, 2, 3, 3]),
            ccs.iter().map(|cc| cc.len()).collect()
        );
    }
}
