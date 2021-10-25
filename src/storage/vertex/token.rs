use std::fmt::Display;
use std::hash::Hash;

/// Describes a token that can act as a representative for a vertex.
///
/// Since a vertex can be any struct(as long as it implements [`VertexDescriptor`]),
/// it's necessary to have a representative for that vertex that conforms to a number of rules.
///
/// A token must:
/// - Have a string representation which in enforced by the `Display` trait.
/// - Be hashable which is enforced by the `Hash` trait.
/// - Be comparable to another token with the same type which is enforced by `PartialEq` and `Eq` trait.
///
/// [`VertexDescriptor`]: crate::storage::vertex::VertexDescriptor
pub trait VertexToken: Display + Hash + PartialEq + Eq {}

impl<T> VertexToken for T where T: Display + Hash + PartialEq + Eq {}
