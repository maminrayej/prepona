use std::hash::Hash;

pub trait VertexToken: Hash + PartialEq + Eq {}

impl<T> VertexToken for T where T: Hash + PartialEq + Eq {}
