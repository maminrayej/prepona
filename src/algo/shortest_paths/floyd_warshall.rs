use std::collections::HashMap;

use anyhow::Result;
use itertools::Itertools;
use magnitude::Magnitude;

use crate::algo::errors::AlgoError;
use crate::common::RealID;
use crate::provide::{Edges, Storage, Vertices};
use crate::storage::edge::Directed;

pub fn floyd_warshall<G, F>(
    graph: &G,
    cost_of: F,
) -> Result<HashMap<(usize, usize), Magnitude<isize>>>
where
    G: Storage<Dir = Directed> + Vertices + Edges,
    F: Fn(usize) -> isize,
{
    let vertex_count = graph.vertex_count();
    let vertices = graph.vertex_tokens().collect_vec();
    let id_map = graph.id_map();

    let mut dist = vec![vec![Magnitude::PosInfinite; vertex_count]; vertex_count];

    for vid in vertices.iter().copied() {
        let v_vid = id_map[RealID::from(vid)];

        dist[v_vid.inner()][v_vid.inner()] = 0.into();
    }

    for uid in vertices.iter().copied() {
        let u_vid = id_map[RealID::from(uid)];

        for eid in graph.outgoing_edges(uid) {
            let (_, vid, _) = graph.edge(eid);
            let v_vid = id_map[RealID::from(vid)];

            dist[u_vid.inner()][v_vid.inner()] = cost_of(eid).into();
        }
    }

    for k in 0..vertex_count {
        for uid in vertices.iter().copied() {
            let u_vid = id_map[RealID::from(uid)];

            for vid in vertices.iter().copied() {
                let v_vid = id_map[RealID::from(vid)];

                if (dist[u_vid.inner()][k] + dist[k][v_vid.inner()]).is_finite()
                    && dist[u_vid.inner()][v_vid.inner()]
                        > dist[u_vid.inner()][k] + dist[k][v_vid.inner()]
                {
                    dist[u_vid.inner()][v_vid.inner()] =
                        dist[u_vid.inner()][k] + dist[k][v_vid.inner()]
                }
            }

            // check for negative cycle
            for vid in vertices.iter().copied() {
                let v_vid = id_map[RealID::from(vid)];

                if dist[v_vid.inner()][v_vid.inner()] < 0.into() {
                    return Err(AlgoError::InvalidArgument(
                        "Graph contains negative cycle".to_string(),
                    )
                    .into());
                }
            }
        }
    }

    let mut distance_map = HashMap::new();

    for uid in vertices.iter().copied() {
        let u_vid = id_map[RealID::from(uid)];

        for vid in vertices.iter().copied() {
            let v_vid = id_map[RealID::from(vid)];

            distance_map.insert((uid, vid), dist[u_vid.inner()][v_vid.inner()]);
        }
    }

    Ok(distance_map)
}
