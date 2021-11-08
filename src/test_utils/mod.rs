use rand::{distributions::Standard, prelude::Distribution, Rng};
use std::{collections::HashSet, hash::Hash};

pub fn get_non_duplicate<T>(set_iter: impl IntoIterator<Item = T>, count: usize) -> Vec<T>
where
    T: Eq + Hash + Clone,
    Standard: Distribution<T>,
{
    let mut set = HashSet::<_>::from_iter(set_iter);

    let mut rng = rand::thread_rng();

    let mut values = Vec::with_capacity(count);

    for _ in 0..count {
        let mut value: T = rng.gen();

        while set.contains(&value) {
            value = rng.gen();
        }

        values.push(value.clone());
        set.insert(value);
    }

    values
}
