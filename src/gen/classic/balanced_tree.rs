use rand::{distributions::Standard, prelude::Distribution};

use crate::gen::Generator;
use crate::provide::{Edges, InitializableStorage, MutEdges, MutVertices, Storage, Vertices};
use crate::storage::edge::Undirected;

use super::FullRAryTreeGenerator;

// TODO: Test it when you've tested full r-ary tree
#[derive(Debug)]
pub struct BalancedTreeGenerator {
    height: usize,
    balance_factor: usize,
}

impl BalancedTreeGenerator {
    pub fn init(height: usize, balance_factor: usize) -> Self {
        BalancedTreeGenerator {
            height,
            balance_factor,
        }
    }
}

impl<S> Generator<S, Undirected> for BalancedTreeGenerator
where
    S: Storage<Dir = Undirected> + InitializableStorage + Vertices + Edges + MutVertices + MutEdges,
    Standard: Distribution<S::V>,
    Standard: Distribution<S::E>,
{
    fn generate(&self) -> S {
        let vertex_count = if self.balance_factor == 1 {
            self.height + 1
        } else {
            (1 - self.balance_factor.pow((self.height + 1) as u32)) / (1 - self.balance_factor)
        };

        FullRAryTreeGenerator::init(vertex_count, self.balance_factor).generate()
    }
}
