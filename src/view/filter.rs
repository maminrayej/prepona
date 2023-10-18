use std::collections::HashSet;

pub trait Filter<T> {
    fn filter(&self, item: &T) -> bool;
}

macro_rules! op {
    ($name: ident, $func: ident, $op: tt) => {
        pub struct $name<F1, F2> {
            f1: F1,
            f2: F2,
        }

        impl<T, F1, F2> Filter<T> for $name<F1, F2>
        where
            F1: Filter<T>,
            F2: Filter<T>,
        {
            fn filter(&self, item: &T) -> bool {
                self.f1.filter(item) $op self.f2.filter(item)
            }
        }

        pub fn $func<F1, F2>(f1: F1, f2: F2) -> $name<F1, F2> {
            $name { f1, f2 }
        }
    };
}

op!(And, and, &&);
op!(Or,  or,  ||);
op!(Xor, xor, ^);

pub struct Not<F> {
    f: F,
}

impl<T, F> Filter<T> for Not<F>
where
    F: Filter<T>,
{
    fn filter(&self, item: &T) -> bool {
        !self.f.filter(item)
    }
}

pub fn not<F>(f: F) -> Not<F> {
    Not { f }
}

impl<T> Filter<T> for HashSet<T>
where
    T: std::hash::Hash + Eq,
{
    fn filter(&self, item: &T) -> bool {
        self.contains(item)
    }
}

impl<T, K> Filter<K> for T
where
    T: Fn(&K) -> bool,
{
    fn filter(&self, item: &K) -> bool {
        (self)(item)
    }
}

#[derive(Debug)]
pub struct AcceptAll;
impl<T> Filter<T> for AcceptAll {
    fn filter(&self, _: &T) -> bool {
        true
    }
}
