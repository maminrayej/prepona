use std::{collections::HashSet, hash::Hash, ops::RangeInclusive};

pub trait Walkable {
    type Walker: Walker<Item = Self>;

    fn walker() -> Self::Walker;
}

impl Walkable for usize {
    type Walker = IterWalker<usize, RangeInclusive<usize>>;

    fn walker() -> Self::Walker {
        IterWalker::init(usize::MIN..=usize::MAX)
    }
}

pub trait Walker {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

pub struct IterWalker<T, I>
where
    I: Iterator<Item = T>,
{
    inner: I,
}

impl<T, I> IterWalker<T, I>
where
    I: Iterator<Item = T>,
{
    pub fn init(iter: I) -> Self {
        IterWalker { inner: iter }
    }
}

impl<T, I> Walker for IterWalker<T, I>
where
    I: Iterator<Item = T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

pub type UsizeTokenProvider = TokenProvider<usize, IterWalker<usize, RangeInclusive<usize>>>;

pub struct TokenProvider<T, W>
where
    T: Walkable + Eq + Hash,
    W: Walker<Item = T>,
{
    reusable_tokens: HashSet<T>,

    walker: W,
}

impl<T, W> TokenProvider<T, W>
where
    T: Walkable<Walker = W> + Eq + Hash,
    W: Walker<Item = T>,
{
    pub fn init() -> Self {
        TokenProvider {
            reusable_tokens: HashSet::new(),
            walker: T::walker(),
        }
    }

    pub fn next(&mut self) -> Option<T> {
        self.walker.next()
    }

    pub fn free(&mut self, token: T) {
        self.reusable_tokens.insert(token);
    }
}
