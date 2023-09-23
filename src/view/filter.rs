pub trait Filter {
    type Item;

    fn select(&self, item: &Self::Item) -> bool;
}

macro_rules! op {
    ($name: ident, $func: ident, $op: tt) => {
        pub struct $name<F1, F2> {
            f1: F1,
            f2: F2,
        }

        impl<T, F1, F2> Filter for $name<F1, F2>
        where
            F1: Filter<Item = T>,
            F2: Filter<Item = T>,
        {
            type Item = T;

            fn select(&self, item: &Self::Item) -> bool {
                self.f1.select(item) $op self.f2.select(item)
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

impl<F> Filter for Not<F>
where
    F: Filter,
{
    type Item = F::Item;

    fn select(&self, item: &Self::Item) -> bool {
        !self.f.select(item)
    }
}

pub fn not<F>(f: F) -> Not<F> {
    Not { f }
}
