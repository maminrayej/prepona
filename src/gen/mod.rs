mod classic;

pub use classic::*;

use crate::{
    provide::{Edges, InitializableStorage, MutStorage, Vertices},
    storage::edge::Direction,
};
use rand::{distributions::Standard, prelude::Distribution};

pub trait Generator<S, Dir: Direction>
where
    S: Edges<Dir = Dir>,
    S: Vertices<Dir = Dir>,
    S: MutStorage,
    S: InitializableStorage<Dir = Dir>,
    Standard: Distribution<S::V>,
    Standard: Distribution<S::E>,
{
    fn generate(&self) -> S;
}
