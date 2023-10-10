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

pub trait Id {
    type Id;

    fn id(&self) -> Self::Id;
}

pub trait Storage {
    type Node: Id<Id = NodeId>;
    type Edge: Id<Id = EdgeId>;
    type Dir: Direction;
    type Map: IdMap;

    fn idmap(&self) -> Self::Map;
}
