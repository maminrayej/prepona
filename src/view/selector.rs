pub trait Selector {
    type Storage;
    type Element;

    fn select(&self, storage: &Self::Storage, element: &Self::Element) -> bool;
}

pub struct AndSelector<S1, S2> {
    selector1: S1,
    selector2: S2,
}

impl<S1, S2> Selector for AndSelector<S1, S2>
where
    S1: Selector,
    S2: Selector<Storage = S1::Storage, Element = S1::Element>,
{
    type Storage = S1::Storage;

    type Element = S1::Element;

    fn select(&self, storage: &Self::Storage, element: &Self::Element) -> bool {
        self.selector1.select(storage, element) && self.selector2.select(storage, element)
    }
}

pub struct OrSelector<S1, S2> {
    selector1: S1,
    selector2: S2,
}

impl<S1, S2> Selector for OrSelector<S1, S2>
where
    S1: Selector,
    S2: Selector<Storage = S1::Storage, Element = S1::Element>,
{
    type Storage = S1::Storage;

    type Element = S1::Element;

    fn select(&self, storage: &Self::Storage, element: &Self::Element) -> bool {
        self.selector1.select(storage, element) || self.selector2.select(storage, element)
    }
}

pub struct NotSelector<S> {
    selector: S,
}

impl<S> Selector for NotSelector<S>
where
    S: Selector,
{
    type Storage = S::Storage;

    type Element = S::Element;

    fn select(&self, storage: &Self::Storage, element: &Self::Element) -> bool {
        !self.selector.select(storage, element)
    }
}

pub struct XORSelector<S1, S2> {
    selector1: S1,
    selector2: S2,
}

impl<S1, S2> Selector for XORSelector<S1, S2>
where
    S1: Selector,
    S2: Selector<Storage = S1::Storage, Element = S1::Element>,
{
    type Storage = S1::Storage;

    type Element = S1::Element;

    fn select(&self, storage: &Self::Storage, element: &Self::Element) -> bool {
        self.selector1.select(storage, element) ^ self.selector2.select(storage, element)
    }
}
