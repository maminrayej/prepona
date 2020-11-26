mod bfs;
mod dfs;

pub use bfs::Bfs;
pub use dfs::{Dfs, DfsListener};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Color {
    White,
    Gray,
    Black,
}
