mod internal {
    pub trait Sealed {}

    impl Sealed for super::Directed {}
    impl Sealed for super::Undirected {}
}

pub trait Direction: internal::Sealed {
    fn is_directed() -> bool;
    fn is_undirected() -> bool {
        !Self::is_directed()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Directed;
impl Direction for Directed {
    fn is_directed() -> bool {
        true
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Undirected;
impl Direction for Undirected {
    fn is_directed() -> bool {
        false
    }
}
