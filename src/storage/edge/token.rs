use std::fmt::Display;
use std::hash::Hash;

trait EdgeToken: Hash + Display + PartialEq + Eq {}

impl EdgeToken for usize {}
