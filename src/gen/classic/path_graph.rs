use rand::{distributions::Standard, prelude::Distribution, thread_rng, Rng};

use crate::provide::{InitializableStorage, MutStorage};

use crate::gen::Generator;

pub struct PathGraphGenerator {
    vertex_count: usize,
}

impl PathGraphGenerator {
    pub fn init(vertex_count: usize) -> Self {
        PathGraphGenerator { vertex_count }
    }
}

impl<S> Generator<S> for PathGraphGenerator
where
    S: InitializableStorage + MutStorage,
    Standard: Distribution<S::V>,
    Standard: Distribution<S::E>,
{
    fn generate(&self) -> S {
        let mut storage = S::init();
        let mut rng = thread_rng();

        let vertex_tokens: Vec<usize> = (0..self.vertex_count)
            .into_iter()
            .map(|_| storage.add_vertex(rng.gen()))
            .collect();

        for vts in vertex_tokens.windows(2) {
            storage.add_edge(vts[0], vts[1], rng.gen());
        }

        storage
    }
}
