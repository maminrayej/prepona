use itertools::Itertools;
use rand::{distributions::Standard, prelude::Distribution, thread_rng, Rng};

use crate::provide::{InitializableStorage, MutStorage};

use super::CycleGraphGenerator;
use crate::gen::Generator;

pub struct CircularLadderGraphGenerator {
    vertex_count: usize,
}

impl CircularLadderGraphGenerator {
    pub fn init(vertex_count: usize) -> Self {
        if vertex_count < 3 {
            panic!("Vertex count must be atleast 3 to form a circular ladder graph")
        }

        CircularLadderGraphGenerator { vertex_count }
    }
}

impl<S> Generator<S> for CircularLadderGraphGenerator
where
    S: InitializableStorage + MutStorage,
    Standard: Distribution<S::V>,
    Standard: Distribution<S::E>,
{
    fn generate(&self) -> S {
        let mut storage: S = CycleGraphGenerator::init(self.vertex_count).generate();

        let mut rng = thread_rng();

        let outer_circle_tokens: Vec<usize> = storage.vertex_tokens().collect();
        let inner_circle_tokens: Vec<usize> = (0..self.vertex_count)
            .into_iter()
            .map(|_| storage.add_vertex(rng.gen()))
            .collect();

        for (src_vt, dst_vt) in inner_circle_tokens.iter().copied().circular_tuple_windows() {
            storage.add_edge(src_vt, dst_vt, rng.gen());
        }

        let vts_pair = outer_circle_tokens
            .iter()
            .copied()
            .zip(inner_circle_tokens.iter().copied());

        for (src_vt, dst_vt) in vts_pair {
            storage.add_edge(src_vt, dst_vt, rng.gen());
        }

        storage
    }
}
