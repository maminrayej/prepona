mod classic;

pub use classic::*;

use crate::provide::{AddEdgeProvider, AddNodeProvider, EmptyStorage, NodeId};

pub trait Generator<S>
where
    S: EmptyStorage + AddNodeProvider + AddEdgeProvider,
{
    fn generate(&self) -> S {
        let mut storage = S::init();

        self.generate_into(&mut storage, 0.into());

        storage
    }

    fn generate_into(&self, storage: &mut S, start_node: NodeId) -> NodeId;
}
