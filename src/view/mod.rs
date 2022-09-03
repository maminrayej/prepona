mod directed_view;
mod generic_view;
mod reverse_view;

pub use directed_view::*;
pub use generic_view::*;
pub use reverse_view::*;

use crate::provide::{EdgeProvider, NodeProvider};

pub trait FrozenView: NodeProvider + EdgeProvider {
    type Graph: NodeProvider + EdgeProvider;

    fn inner(&self) -> &Self::Graph;
}
