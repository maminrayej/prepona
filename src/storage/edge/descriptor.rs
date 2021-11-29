use super::Direction;

pub trait EdgeDescriptor<const DIR: bool>: PartialEq + Eq + Direction<DIR> {}
