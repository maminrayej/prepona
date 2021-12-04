use rand::{distributions::Standard, prelude::Distribution, thread_rng, Rng};

use crate::provide::{InitializableStorage, MutStorage};

use super::Generator;

pub struct EmptyGraphGenerator {
    vertex_count: usize,
}

impl EmptyGraphGenerator {
    pub fn init(vertex_count: usize) -> Self {
        EmptyGraphGenerator { vertex_count }
    }
}

impl<S> Generator<S> for EmptyGraphGenerator
where
    S: InitializableStorage + MutStorage,
    Standard: Distribution<S::V>,
    Standard: Distribution<S::E>,
{
    fn generate(&self) -> S {
        let mut storage = S::init();
        let mut rng = thread_rng();

        for _ in 0..self.vertex_count {
            storage.add_vertex(rng.gen());
        }

        storage
    }
}
