mod dir;
pub use dir::*;

mod node;
pub use node::*;

mod edge;
pub use edge::*;

cfg_parallel! {
    mod node_par;
    pub use node_par::*;

    mod edge_par;
    pub use edge_par::*;
}

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
