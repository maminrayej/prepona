mod classic;

pub use classic::*;

use crate::{
    provide::{Edges, Storage, MutEdges, MutVertices, Vertices},
    storage::edge::Direction,
};
use rand::{distributions::Standard, prelude::Distribution};

pub trait Generator<S, Dir: Direction>
where
    S: Edges<Dir = Dir>,
    S: Vertices<Dir = Dir>,
    S: MutVertices + MutEdges,
    S: Storage<Dir = Dir>,
    Standard: Distribution<S::V>,
    Standard: Distribution<S::E>,
{
    fn generate(&self) -> S;
}
