use itertools::Itertools;
use rand::{distributions::Standard, prelude::Distribution, thread_rng, Rng};

use crate::provide::{InitializableStorage, MutStorage};

use crate::gen::Generator;

pub struct CycleGraphGenerator {
    vertex_count: usize,
}

impl CycleGraphGenerator {
    pub fn init(vertex_count: usize) -> Self {
        if vertex_count < 3 {
            panic!("Vertex count must be atleast 3 to form a cycle graph")
        }

        CycleGraphGenerator { vertex_count }
    }
}

impl<S> Generator<S> for CycleGraphGenerator
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

        for (src_vt, dst_vt) in vertex_tokens.into_iter().circular_tuple_windows() {
            storage.add_edge(src_vt, dst_vt, rng.gen());
        }

        storage
    }
}
