use std::fmt::Display;
use std::hash::Hash;

/// Describes a token that can act as a representative for an edge.
///
/// Since an edge can be any struct(as long as it implements [`EdgeDescriptor`]),
/// it's necessary to have a representative for that edge that conforms to a number of rules.
///
/// # Required traits
/// * `Display`: A token must have a string representation.
/// * `Hash`: A token must be hashable.
/// * `PartialEq`, `Eq`: A token must be comparable to another token with the same type.
///
/// [`EdgeDescriptor`]: crate::storage::edge::EdgeDescriptor
pub trait EdgeToken: Display + Hash + PartialEq + Eq {}

impl<T> EdgeToken for T where T: Hash + PartialEq + Eq + Display {}
