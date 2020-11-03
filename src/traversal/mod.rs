mod bfs;
mod dfs;

pub use bfs::Bfs;
pub use dfs::Dfs;

#[derive(Copy, Clone, PartialEq)]
pub enum Color {
    White,
    Gray,
    Black,
}
