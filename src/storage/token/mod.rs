use std::{collections::HashSet, ops::RangeInclusive};

#[derive(Debug)]
pub struct UsizeTokenProvider {
    reusable_tokens: HashSet<usize>,
    inner: RangeInclusive<usize>,
}

impl UsizeTokenProvider {
    pub fn init() -> Self {
        UsizeTokenProvider {
            reusable_tokens: HashSet::new(),
            inner: (usize::MIN..=usize::MAX),
        }
    }

    pub fn get(&mut self) -> Option<usize> {
        self.inner.next()
    }

    pub fn free(&mut self, token: usize) {
        self.reusable_tokens.insert(token);
    }
}

#[cfg(test)]
mod test {
    use super::UsizeTokenProvider;

    impl Clone for UsizeTokenProvider {
        fn clone(&self) -> Self {
            Self {
                reusable_tokens: self.reusable_tokens.clone(),
                inner: self.inner.clone(),
            }
        }
    }
}
