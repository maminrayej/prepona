mod dir;
pub use dir::*;

mod map;
pub use map::*;

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

pub trait Storage {
    type Node;
    type Edge;
    type Dir: Direction;
    type Map: IdMap;
}
