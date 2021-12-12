use std::collections::HashMap;

use itertools::Itertools;

// TODO: provide new types for real and virtual id
// TODO: Implement Index<RealId> -> VirtualId
// TODO: Implement Index<VirtualId> -> RealId

pub struct IdMap {
    real_to_virt: HashMap<usize, usize>,
    virt_to_real: Vec<usize>,
}

impl IdMap {
    pub fn init(ids: impl Iterator<Item = usize>) -> Self {
        let virt_to_real = ids.collect_vec();
        let real_to_virt: HashMap<usize, usize> =
            virt_to_real.iter().copied().enumerate().collect();

        IdMap {
            real_to_virt,
            virt_to_real,
        }
    }

    pub fn virt_of(&self, real_id: usize) -> usize {
        self.real_to_virt[&real_id]
    }

    pub fn real_of(&self, virt_id: usize) -> usize {
        self.virt_to_real[virt_id]
    }
}
