use rand::{distributions::Standard, prelude::Distribution};

use crate::provide::{InitializableStorage, MutStorage};

use super::Generator;

pub struct NullGraphGenerator;

impl<S> Generator<S> for NullGraphGenerator
where
    S: InitializableStorage + MutStorage,
    Standard: Distribution<S::V>,
    Standard: Distribution<S::E>,
{
    fn generate(&self) -> S {
        S::init()
    }
}
