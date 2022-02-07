use std::collections::{HashMap, HashSet};

use anyhow::Result;
use magnitude::Magnitude;

use crate::algo::errors::AlgoError;
use crate::common::{RealID, VirtID};
use crate::provide::{Edges, Storage, Vertices};
use crate::storage::edge::Directed;
use crate::view::GenericView;

pub fn bellman_ford<G, F>(
    graph: &G,
    cost_of: F,
) -> Result<(GenericView<'_, G>, HashMap<usize, isize>)>
where
    G: Storage<Dir = Directed> + Vertices + Edges,
    F: Fn(usize) -> isize,
{
    let vertex_count = graph.vertex_count();
    let id_map = graph.id_map();

    let mut costs = vec![Magnitude::PosInfinite; vertex_count];
    let mut visited_edges = HashSet::new();

    for _ in 1..vertex_count {
        for eid in graph.edge_tokens() {
            let (sid, did, _) = graph.edge(eid);

            let s_vid = id_map[RealID::from(sid)];
            let d_vid = id_map[RealID::from(did)];

            let s_cost = costs[s_vid.inner()];
            let d_cost = costs[d_vid.inner()];

            let new_cost = s_cost + cost_of(eid).into();
            if new_cost.is_finite() && d_cost > new_cost {
                costs[d_vid.inner()] = new_cost;
                visited_edges.insert(eid);
            }
        }
    }

    for eid in graph.edge_tokens() {
        let (sid, did, _) = graph.edge(eid);

        let s_vid = id_map[RealID::from(sid)];
        let d_vid = id_map[RealID::from(did)];

        let s_cost = costs[s_vid.inner()];
        let d_cost = costs[d_vid.inner()];

        let new_cost = s_cost + cost_of(eid).into();
        if new_cost.is_finite() && d_cost > new_cost {
            return Err(
                AlgoError::InvalidArgument("Graph contains negative cycle".to_string()).into(),
            );
        }
    }

    let tree_view = GenericView::init(
        graph,
        |vid| {
            let v_vid = id_map[RealID::from(vid)];
            costs[v_vid.inner()].is_finite()
        },
        |eid| visited_edges.contains(&eid),
    );

    let cost_map = costs
        .iter()
        .enumerate()
        .filter_map(|(index, cost)| {
            let v_vid = VirtID::from(index);
            let v_rid = id_map[v_vid];

            if costs[v_vid.inner()].is_finite() {
                Some((v_rid.inner(), cost.unwrap()))
            } else {
                None
            }
        })
        .collect();

    Ok((tree_view, cost_map))
}
