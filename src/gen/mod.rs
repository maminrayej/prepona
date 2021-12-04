mod balanced_tree;
mod complete_graph;
mod cycle_graph;
mod empty_graph;
mod full_rary_tree;
mod ladder_graph;
mod null_graph;
mod path_graph;
mod star_graph;
mod wheel_graph;

pub use balanced_tree::*;
pub use complete_graph::*;
pub use cycle_graph::*;
pub use empty_graph::*;
pub use full_rary_tree::*;
pub use ladder_graph::*;
pub use null_graph::*;
pub use path_graph::*;
pub use star_graph::*;
pub use wheel_graph::*;

use crate::provide::{InitializableStorage, MutStorage};
use rand::{distributions::Standard, prelude::Distribution};

pub trait Generator<S>
where
    S: InitializableStorage + MutStorage,
    Standard: Distribution<S::V>,
    Standard: Distribution<S::E>,
{
    fn generate(&self) -> S;
}
