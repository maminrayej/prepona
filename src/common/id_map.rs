use std::{collections::HashMap, ops::Index};

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RealID(usize);

impl From<usize> for RealID {
    fn from(value: usize) -> Self {
        RealID(value)
    }
}

impl RealID {
    pub fn inner(&self) -> usize {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VirtID(usize);

impl VirtID {
    pub fn inner(&self) -> usize {
        self.0
    }
}

impl From<usize> for VirtID {
    fn from(value: usize) -> Self {
        VirtID(value)
    }
}

pub struct IdMap {
    real_to_virt: HashMap<RealID, VirtID>,
    virt_to_real: Vec<RealID>,
}

impl IdMap {
    pub fn init(ids: impl Iterator<Item = usize>) -> Self {
        let virt_to_real = ids.map(|rid| rid.into()).collect_vec();

        let real_to_virt: HashMap<RealID, VirtID> = virt_to_real
            .iter()
            .copied()
            .enumerate()
            .map(|(vid, real_id)| (real_id, vid.into()))
            .collect();

        IdMap {
            real_to_virt,
            virt_to_real,
        }
    }
}

impl Index<RealID> for IdMap {
    type Output = VirtID;

    fn index(&self, index: RealID) -> &Self::Output {
        &self.real_to_virt[&index]
    }
}

impl Index<VirtID> for IdMap {
    type Output = RealID;

    fn index(&self, index: VirtID) -> &Self::Output {
        &self.virt_to_real[index.0]
    }
}
