use rand::{distributions::Standard, prelude::Distribution, thread_rng, Rng};

use crate::provide::{Edges, InitializableStorage, MutStorage, Vertices};

use crate::gen::Generator;
use crate::storage::edge::Undirected;

// TODO: add property test when you have more algorithms in your disposal
#[derive(Debug)]
pub struct FullRAryTreeGenerator {
    vertex_count: usize,
    balance_factor: usize,
}

impl FullRAryTreeGenerator {
    pub fn init(vertex_count: usize, balance_factor: usize) -> Self {
        FullRAryTreeGenerator {
            vertex_count,
            balance_factor,
        }
    }
}

impl<S> Generator<S, Undirected> for FullRAryTreeGenerator
where
    S: Edges<Dir = Undirected>,
    S: Vertices<Dir = Undirected>,
    S: MutStorage,
    S: InitializableStorage<Dir = Undirected>,
    Standard: Distribution<S::V>,
    Standard: Distribution<S::E>,
{
    fn generate(&self) -> S {
        let mut storage = S::init();
        let mut rng = thread_rng();

        let mut vertex_tokens: Vec<usize> = (0..self.vertex_count)
            .into_iter()
            .map(|_| storage.add_vertex(rng.gen()))
            .collect();

        let mut parent_stack = vec![vertex_tokens.pop().unwrap()];

        while !parent_stack.is_empty() {
            let src_vt = parent_stack.pop().unwrap();

            for _ in 0..self.balance_factor {
                if let Some(dst_vt) = vertex_tokens.pop() {
                    storage.add_edge(src_vt, dst_vt, rng.gen());
                    parent_stack.push(dst_vt);
                }
            }
        }

        storage
    }
}
