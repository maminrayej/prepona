/// Describes what is expected from a vertex.
pub trait VertexDescriptor: PartialEq + Eq {}

impl<T: PartialEq + Eq> VertexDescriptor for T {}
