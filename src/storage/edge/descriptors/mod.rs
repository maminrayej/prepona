mod edge;
mod hyperedge;

pub use edge::Edge;
pub use hyperedge::{HashHyperedge, Hyperedge, UnorderedSet};

use super::direction::EdgeDirection;
use crate::storage::vertex::VertexToken;

pub trait EdgeDescriptor<Dir: EdgeDirection, VT: VertexToken>: PartialEq + Eq {
    fn get_sources(&self) -> Box<dyn Iterator<Item = &VT> + '_>;
    fn get_destinations(&self) -> Box<dyn Iterator<Item = &VT> + '_>;

    fn is_directed() -> bool {
        Dir::is_directed()
    }
}
