mod dfs;
pub use dfs::*;

mod bfs;
pub use bfs::*;

const UNKNOWN: usize = usize::MAX;

pub type VisitFlow = std::ops::ControlFlow<(), Continue>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Continue {
    Noop,
    Prune,
}

mod internal {
    macro_rules! control {
        ($e: expr) => {
            match $e? {
                crate::algo::visit::Continue::Noop => {}
                crate::algo::visit::Continue::Prune => continue,
            }
        };
    }

    pub(super) use control;
}
use internal::control;
