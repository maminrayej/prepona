use std::collections::HashMap;

use itertools::Itertools;

use crate::provide::{Edges, Storage, Vertices};
use crate::storage::edge::{Directed, Direction};

pub fn degree_centrality<G>(graph: &G) -> HashMap<usize, f64>
where
    G: Storage + Vertices + Edges,
{
    let s = 1.0 / (graph.vertex_count() as f64 - 1.0);

    graph
        .vertex_tokens()
        .map(|vid| {
            let mut d = if G::Dir::is_undirected() {
                graph.ingoing_edges(vid).count()
            } else {
                graph
                    .ingoing_edges(vid)
                    .chain(graph.outgoing_edges(vid))
                    .count()
            };

            if graph.neighbors(vid).contains(&vid) {
                d += 1;
            }

            (vid, d as f64 * s)
        })
        .collect()
}

pub fn in_degree_centrality<G>(graph: &G) -> HashMap<usize, f64>
where
    G: Storage<Dir = Directed> + Vertices + Edges,
{
    let s = 1.0 / (graph.vertex_count() as f64 - 1.0);

    graph
        .vertex_tokens()
        .map(|vid| {
            let d = graph.ingoing_edges(vid).count();

            (vid, d as f64 * s)
        })
        .collect()
}

pub fn out_degree_centrality<G>(graph: &G) -> HashMap<usize, f64>
where
    G: Storage<Dir = Directed> + Vertices + Edges,
{
    let s = 1.0 / (graph.vertex_count() as f64 - 1.0);

    graph
        .vertex_tokens()
        .map(|vid| {
            let d = graph.outgoing_edges(vid).count();

            (vid, d as f64 * s)
        })
        .collect()
}
