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
        self.virt_to_real.get(&virt_id).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        // Given: An empty id map.
        let id_map = IdMap::init();

        // When: Doing nothing.

        // Then:
        assert!(id_map.virt_to_real.is_empty());
        assert!(id_map.real_to_virt.is_empty());
    }

    #[test]
    fn put_real_to_virt() {
        // Given: An empty id map.
        let mut id_map = IdMap::init();

        // When: Adding a mapping from 1(real) to 2(virtual)
        id_map.put_real_to_virt(1, 2);

        // Then:
        assert_eq!(*id_map.real_to_virt.get(&1).unwrap(), 2);
        assert!(id_map.virt_to_real.is_empty());
    }

    #[test]
    fn put_virt_to_real() {
        // Given: An empty id map.
        let mut id_map = IdMap::init();

        // When: Adding a mapping from 1(virtual) to 2(real)
        id_map.put_virt_to_real(1, 2);

        // Then:
        assert_eq!(*id_map.virt_to_real.get(&1).unwrap(), 2);
        assert!(id_map.real_to_virt.is_empty());
    }

    #[test]
    fn get_real_to_virt() {
        // Given: An empty id map.
        let mut id_map = IdMap::init();

        // When: Adding a mapping from 1(real) to 2(virtual).
        id_map.put_real_to_virt(1, 2);

        // Then: Api must return the mapping.
        assert_eq!(id_map.get_real_to_virt(1).unwrap(), 2);
    }

    #[test]
    fn get_virt_to_real() {
        // Given: An empty id map.
        let mut id_map = IdMap::init();

        // When: Adding a mapping between from 1(virtual) to 2(real).
        id_map.put_virt_to_real(1, 2);

        // Then: Api must return the mapping.
        assert_eq!(id_map.get_virt_to_real(1).unwrap(), 2);
    }
}
