use super::dir::Direction;

pub trait Storage {
    type Dir: Direction;
}

pub trait EmptyStorage: Storage {
    fn init() -> Self;
}
