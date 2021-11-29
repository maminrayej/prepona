pub struct DynIter<'a, T> {
    inner: Box<dyn Iterator<Item = T> + 'a>,
}

impl<'a, T> DynIter<'a, T> {
    pub fn init(iter: impl Iterator<Item = T> + 'a) -> Self {
        DynIter {
            inner: Box::new(iter),
        }
    }
}

impl<'a, T> Iterator for DynIter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}
