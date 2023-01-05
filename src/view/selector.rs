pub trait Selector {
    type Storage;
    type Element;

    fn select(&self, storage: &Self::Storage, element: &Self::Element) -> bool;

    fn and<'a, S>(&'a self, rhs: &'a S) -> AndSelector<Self, S> {
        AndSelector {
            sel1: self,
            sel2: rhs,
        }
    }

    fn or<'a, S>(&'a self, rhs: &'a S) -> OrSelector<Self, S> {
        OrSelector {
            sel1: self,
            sel2: rhs,
        }
    }

    fn xor<'a, S>(&'a self, rhs: &'a S) -> XORSelector<Self, S> {
        XORSelector {
            sel1: self,
            sel2: rhs,
        }
    }

    fn not(&self) -> NotSelector<Self> {
        NotSelector { sel: self }
    }
}

pub struct AndSelector<'a, S1: ?Sized, S2: ?Sized> {
    sel1: &'a S1,
    sel2: &'a S2,
}

impl<'a, S1, S2> Selector for AndSelector<'a, S1, S2>
where
    S1: Selector,
    S2: Selector<Storage = S1::Storage, Element = S1::Element>,
{
    type Storage = S1::Storage;

    type Element = S1::Element;

    fn select(&self, storage: &Self::Storage, element: &Self::Element) -> bool {
        self.sel1.select(storage, element) && self.sel2.select(storage, element)
    }
}

pub struct OrSelector<'a, S1: ?Sized, S2: ?Sized> {
    sel1: &'a S1,
    sel2: &'a S2,
}

impl<'a, S1, S2> Selector for OrSelector<'a, S1, S2>
where
    S1: Selector,
    S2: Selector<Storage = S1::Storage, Element = S1::Element>,
{
    type Storage = S1::Storage;

    type Element = S1::Element;

    fn select(&self, storage: &Self::Storage, element: &Self::Element) -> bool {
        self.sel1.select(storage, element) || self.sel2.select(storage, element)
    }
}

pub struct NotSelector<'a, S: ?Sized> {
    sel: &'a S,
}

impl<'a, S> Selector for NotSelector<'a, S>
where
    S: Selector,
{
    type Storage = S::Storage;

    type Element = S::Element;

    fn select(&self, storage: &Self::Storage, element: &Self::Element) -> bool {
        !self.sel.select(storage, element)
    }
}

pub struct XORSelector<'a, S1: ?Sized, S2: ?Sized> {
    sel1: &'a S1,
    sel2: &'a S2,
}

impl<'a, S1, S2> Selector for XORSelector<'a, S1, S2>
where
    S1: Selector,
    S2: Selector<Storage = S1::Storage, Element = S1::Element>,
{
    type Storage = S1::Storage;

    type Element = S1::Element;

    fn select(&self, storage: &Self::Storage, element: &Self::Element) -> bool {
        self.sel1.select(storage, element) ^ self.sel2.select(storage, element)
    }
}
