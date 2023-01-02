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

mod map;
pub use map::*;

mod error;
pub use error::*;

pub trait Storage {
    type Dir: Direction;
    type Map: IDMap;
}
