use crate::common::RealID;
use crate::provide::{Edges, Storage, Vertices};
use crate::storage::edge::Directed;
use crate::view::UndirectedView;

use super::connected_components;

pub fn weakly_connected_components<G>(graph: &G) -> Vec<Vec<RealID>>
where
    G: Storage<Dir = Directed> + Vertices + Edges,
{
    let undir_view = UndirectedView::init(graph, |_| true, |_| true);

    connected_components(&undir_view)
}

pub fn number_weakly_connected_components<G>(graph: &G) -> usize
where
    G: Storage<Dir = Directed> + Vertices + Edges,
{
    weakly_connected_components(graph).len()
}

pub fn is_weakly_connected<G>(graph: &G) -> bool
where
    G: Storage<Dir = Directed> + Vertices + Edges,
{
    number_weakly_connected_components(graph) == 1
}
