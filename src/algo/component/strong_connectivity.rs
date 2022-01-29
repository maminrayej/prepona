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
                    let p_rid = RealID::from(state.parent[&v_vid.inner()]); // it can panic
                    let p_vid = state.id_map[p_rid];
                    let p_low_id = low_id[p_vid.inner()];

                    low_id[p_vid.inner()] = std::cmp::min(v_low_id, p_low_id);
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
                _ => unreachable!(),
            },
            _ => unreachable!(),
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
