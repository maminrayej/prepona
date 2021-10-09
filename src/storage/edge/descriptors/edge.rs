use super::EdgeDescriptor;
use crate::storage::edge::direction::EdgeDirection;
use crate::storage::vertex::VertexToken;
use std::marker::PhantomData;

pub struct Edge<Dir: EdgeDirection, VT: VertexToken> {
    source_vertex_token: VT,
    destination_vertex_token: VT,

    phantom_dir: PhantomData<Dir>,
}

impl<Dir: EdgeDirection, VT: VertexToken> Edge<Dir, VT> {
    pub fn init(source_vertex_token: VT, destination_vertex_token: VT) -> Self {
        Edge {
            source_vertex_token,
            destination_vertex_token,
            phantom_dir: PhantomData,
        }
    }
}

impl<Dir: EdgeDirection, VT: VertexToken> PartialEq for Edge<Dir, VT> {
    fn eq(&self, other: &Self) -> bool {
        self.source_vertex_token == other.source_vertex_token
            && self.destination_vertex_token == other.destination_vertex_token
            && self.phantom_dir == other.phantom_dir
    }
}

impl<Dir: EdgeDirection, VT: VertexToken> Eq for Edge<Dir, VT> {}

impl<Dir: EdgeDirection, VT: VertexToken> EdgeDescriptor<Dir, VT> for Edge<Dir, VT> {
    fn get_sources(&self) -> Box<dyn Iterator<Item = &VT> + '_> {
        Box::new(std::iter::once(&self.source_vertex_token))
    }

    fn get_destinations(&self) -> Box<dyn Iterator<Item = &VT> + '_> {
        Box::new(std::iter::once(&self.destination_vertex_token))
    }
}
