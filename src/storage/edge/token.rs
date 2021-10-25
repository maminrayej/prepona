use std::fmt::Display;
use std::hash::Hash;

/// Describes a token that can act as a representative for an edge.
///
/// Since an edge can be any struct(as long as it implements [`EdgeDescriptor`]),
/// it's necessary to have a representative for that edge that conforms to a number of rules.
///
/// A token must:
/// - Have a string representation which in enforced by the `Display` trait.
/// - Be hashable which is enforced by the `Hash` trait.
/// - Be comparable to another token with the same type which is enforced by `PartialEq` and `Eq` trait.
///
/// [`EdgeDescriptor`]: crate::storage::edge::EdgeDescriptor
pub trait EdgeToken: Display + Hash + PartialEq + Eq {}

impl<T> EdgeToken for T where T: Hash + PartialEq + Eq + Display {}
