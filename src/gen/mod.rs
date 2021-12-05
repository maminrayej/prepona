mod classic;

pub use classic::*;

use crate::provide::{InitializableStorage, MutStorage};
use rand::{distributions::Standard, prelude::Distribution};

pub trait Generator<S>
where
    S: InitializableStorage + MutStorage,
    Standard: Distribution<S::V>,
    Standard: Distribution<S::E>,
{
    fn generate(&self) -> S;
}
