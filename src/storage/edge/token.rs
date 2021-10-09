use std::fmt::Display;
use std::hash::Hash;

pub trait EdgeToken: Display + Hash + PartialEq + Eq {}

impl<T> EdgeToken for T where T: Hash + PartialEq + Eq + Display {}
