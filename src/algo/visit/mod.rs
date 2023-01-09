mod dfs;
pub use dfs::*;

mod bfs;
pub use bfs::*;

pub type ControlFlow = std::ops::ControlFlow<(), Continue>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Continue {
    Prune,
    Noop,
}

mod macros {
    macro_rules! on_event {
        ($res: expr) => {
            match $res? {
                $crate::algo::visit::Continue::Prune => continue,
                $crate::algo::visit::Continue::Noop => {}
            }
        };
    }

    pub(super) use on_event;
}
