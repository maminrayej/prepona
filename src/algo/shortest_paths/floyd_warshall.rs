use std::collections::HashMap;

use anyhow::Result;
use itertools::Itertools;
use magnitude::Magnitude;

use crate::algo::errors::AlgoError;
use crate::common::RealID;
use crate::provide::{Edges, Storage, Vertices};
use crate::storage::edge::Directed;

pub fn floyd_warshall<G, F>(graph: &G, cost_of: F) -> Result<HashMap<(usize, usize), isize>>
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

                let cost_through_k = dist[u_vid.inner()][k] + dist[k][v_vid.inner()];
                let direct_cost = dist[u_vid.inner()][v_vid.inner()];

                if cost_through_k.is_finite() && cost_through_k < direct_cost {
                    dist[u_vid.inner()][v_vid.inner()] = cost_through_k;
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

            if let Magnitude::Finite(cost) = dist[u_vid.inner()][v_vid.inner()] {
                distance_map.insert((uid, vid), cost);
            }
        }
    }

    Ok(distance_map)
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use quickcheck_macros::quickcheck;

    use crate::algo::shortest_paths::{dijkstra, floyd_warshall};
    use crate::common::RealID;
    use crate::gen::{CompleteGraphGenerator, Generator, PathGraphGenerator};
    use crate::provide::{Edges, Storage, Vertices};
    use crate::storage::edge::Directed;
    use crate::storage::AdjMap;

    fn check_against_dijkstra<G>(graph: &G)
    where
        G: Storage<Dir = Directed> + Vertices + Edges,
    {
        let vertices = graph.vertex_tokens().collect_vec();

        let floyd_warshall_res = floyd_warshall(graph, |_| 1).unwrap();

        for src_id in vertices.iter().copied() {
            let (_, dijkstra_res) = dijkstra(graph, RealID::from(src_id), None, |_| 1);

            assert_eq!(
                floyd_warshall_res
                    .iter()
                    .filter(|((sid, _), _)| *sid == src_id)
                    .count(),
                dijkstra_res.values().count()
            );

            for dst_id in vertices.iter().copied() {
                assert_eq!(
                    floyd_warshall_res.get(&(src_id, dst_id)).copied(),
                    dijkstra_res.get(&dst_id).map(|cost| *cost as isize)
                );
            }
        }
    }

    #[quickcheck]
    fn prop_floyd_warshall_on_path_graph(generator: PathGraphGenerator) {
        let graph: AdjMap<(), (), Directed> = generator.generate();

        check_against_dijkstra(&graph);
    }

    #[quickcheck]
    fn prop_floyd_warshall_on_complete_graph(generator: CompleteGraphGenerator) {
        let graph: AdjMap<(), (), Directed> = generator.generate();

        check_against_dijkstra(&graph);
    }
}
