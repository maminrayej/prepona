use std::fmt::Display;
use std::hash::Hash;

pub trait VertexToken: Display + Hash + PartialEq + Eq {}

impl<T> VertexToken for T where T: Display + Hash + PartialEq + Eq {}
