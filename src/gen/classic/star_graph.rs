use rand::{distributions::Standard, prelude::Distribution, thread_rng, Rng};

use crate::provide::{InitializableStorage, MutStorage};

use crate::gen::Generator;

pub struct StarGraphGenerator {
    vertex_count: usize,
}

impl StarGraphGenerator {
    pub fn init(vertex_count: usize) -> Self {
        if vertex_count < 3 {
            panic!("Vertex count must be atleast 3 to form a star graph")
        }

        StarGraphGenerator { vertex_count }
    }
}

impl<S> Generator<S> for StarGraphGenerator
where
    S: InitializableStorage + MutStorage,
    Standard: Distribution<S::V>,
    Standard: Distribution<S::E>,
{
    fn generate(&self) -> S {
        let mut storage = S::init();
        let mut rng = thread_rng();

        let vertex_tokens: Vec<usize> = (0..self.vertex_count - 1)
            .into_iter()
            .map(|_| storage.add_vertex(rng.gen()))
            .collect();

        let center_vt = storage.add_vertex(rng.gen());

        for vt in vertex_tokens {
            storage.add_edge(center_vt, vt, rng.gen());
        }

        storage
    }
}
