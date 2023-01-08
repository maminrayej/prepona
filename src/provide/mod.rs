mod dir;
pub use dir::*;

mod node;
pub use node::*;

#[cfg(feature = "parallel")]
mod node_par;
#[cfg(feature = "parallel")]
pub use node_par::*;

mod edge;
pub use edge::*;

#[cfg(feature = "parallel")]
mod edge_par;
#[cfg(feature = "parallel")]
pub use edge_par::*;

mod map;
pub use map::*;

mod error;
pub use error::*;

pub trait Storage {
    type Dir: Direction;
    type Map: IDMap;

    fn idmap(&self) -> Self::Map;
}

pub trait EmptyStorage: Storage {
    fn init() -> Self;
}
