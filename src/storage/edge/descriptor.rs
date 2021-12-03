pub trait EdgeDescriptor: PartialEq + Eq {}

impl<T> EdgeDescriptor for T where T: PartialEq + Eq {}
