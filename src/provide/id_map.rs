use std::collections::HashMap;

pub struct IdMap {
    real_to_virt: HashMap<usize, usize>,
    virt_to_real: HashMap<usize, usize>,
}

impl IdMap {
    pub fn init() -> Self {
        IdMap {
            real_to_virt: HashMap::new(),
            virt_to_real: HashMap::new(),
        }
    }

    pub fn put_real_to_virt(&mut self, real_id: usize, virt_id: usize) {
        self.real_to_virt.insert(real_id, virt_id);
    }

    pub fn put_virt_to_real(&mut self, virt_id: usize, real_id: usize) {
        self.virt_to_real.insert(virt_id, real_id);
    }

    pub fn get_real_to_virt(&self, real_id: usize) -> Option<usize> {
        self.real_to_virt.get(&real_id).copied()
    }

    pub fn get_virt_to_real(&self, virt_id: usize) -> Option<usize> {
        self.real_to_virt.get(&virt_id).copied()
    }
}
