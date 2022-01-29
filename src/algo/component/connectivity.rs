use anyhow::Result;

use crate::algo::errors::AlgoError;
use crate::algo::traversal::{ControlFlow, Event, DFS};
use crate::common::RealID;
use crate::provide::{Storage, Vertices};
use crate::storage::edge::Undirected;

pub fn connected_components<G>(graph: &G) -> Vec<Vec<RealID>>
where
    G: Storage<Dir = Undirected> + Vertices,
{
    let mut ccs = vec![];

    let mut current = vec![];

    DFS::init(graph).execute(|event| {
        match event {
            Event::Discover(_, vid) => current.push(vid),
            Event::End(_) => {
                ccs.push(current.clone());
                current.clear();
            }
            _ => {}
        }

        ControlFlow::Continue
    });

    ccs
}

pub fn number_connected_components<G>(graph: &G) -> usize
where
    G: Storage<Dir = Undirected> + Vertices,
{
    connected_components(graph).len()
}

pub fn is_connected<G>(graph: &G) -> Result<bool>
where
    G: Storage<Dir = Undirected> + Vertices,
{
    if graph.vertex_count() == 0 {
        Err(
            AlgoError::UndefinedConcept("Connectivity is undefined for null graph".to_string())
                .into(),
        )
    } else {
        Ok(number_connected_components(graph) == 1)
    }
}
