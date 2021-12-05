use rand::{distributions::Standard, prelude::Distribution};

use crate::gen::Generator;
use crate::provide::{InitializableStorage, MutStorage};

use super::FullRAryTreeGenerator;

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

impl<S> Generator<S> for BalancedTreeGenerator
where
    S: InitializableStorage + MutStorage,
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
