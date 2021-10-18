pub trait Direction<const DIR: bool> {
    fn is_directed() -> bool {
        DIR
    }

    fn is_undirected() -> bool {
        !DIR
    }
}

pub struct Directed;
impl Direction<true> for Directed {}

pub struct Undirected;
impl Direction<false> for Undirected {}
