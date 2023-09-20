mod internal {
    pub trait Sealed {}

    impl Sealed for super::Directed {}
    impl Sealed for super::Undirected {}
}

pub trait Direction: internal::Sealed {
    fn is_directed(&self) -> bool;
    fn is_undirected(&self) -> bool {
        !self.is_directed()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Directed;
impl Direction for Directed {
    fn is_directed(&self) -> bool {
        true
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Undirected;
impl Direction for Undirected {
    fn is_directed(&self) -> bool {
        false
    }
}
