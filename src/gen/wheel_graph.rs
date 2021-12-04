use rand::{distributions::Standard, prelude::Distribution, thread_rng, Rng};

use crate::provide::{InitializableStorage, MutStorage};

use super::{CycleGraphGenerator, Generator};

pub struct WheelGraphGenerator {
    vertex_count: usize,
}

impl WheelGraphGenerator {
    pub fn init(vertex_count: usize) -> Self {
        if vertex_count < 4 {
            panic!("Vertex count must be atleast 4 to form a wheel graph")
        }

        WheelGraphGenerator { vertex_count }
    }
}

impl<S> Generator<S> for WheelGraphGenerator
where
    S: InitializableStorage + MutStorage,
    Standard: Distribution<S::V>,
    Standard: Distribution<S::E>,
{
    fn generate(&self) -> S {
        let mut storage: S = CycleGraphGenerator::init(self.vertex_count - 1).generate();
        let mut rng = thread_rng();

        let vertex_tokens: Vec<usize> = storage.vertex_tokens().collect();
        let universal_vt = storage.add_vertex(rng.gen());

        for other_vt in vertex_tokens {
            storage.add_edge(universal_vt, other_vt, rng.gen());
        }

        storage
    }
}
