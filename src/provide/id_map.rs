use std::collections::HashMap;

/// Stores a two-way mapping between set of real and virtual ids.
pub struct IdMap {
    real_to_virt: HashMap<usize, usize>,
    virt_to_real: HashMap<usize, usize>,
}

impl IdMap {
    /// # Arguments
    /// `entries_count`: Number of ids that are gonna be mapped.
    ///
    /// # Returns
    /// An empty id map.
    pub fn init(entries_count: usize) -> Self {
        IdMap {
            real_to_virt: HashMap::with_capacity(entries_count),
            virt_to_real: HashMap::with_capacity(entries_count),
        }
    }

    /// Inserts a mapping from `real_id` to `virt_id`.
    ///
    /// # Arguments
    /// * `real_id`: Real id of the mapping.
    /// * `virt_id`: Virtual id of the mapping.
    pub fn put_real_to_virt(&mut self, real_id: usize, virt_id: usize) {
        self.real_to_virt.insert(real_id, virt_id);
    }

    /// Inserts a mapping from `real_id` to `virt_id`.
    ///
    /// # Arguments
    /// * `virt_id`: Virtual id of the mapping.
    /// * `real_id`: Real id of the mapping.
    pub fn put_virt_to_real(&mut self, virt_id: usize, real_id: usize) {
        self.virt_to_real.insert(virt_id, real_id);
    }

    /// # Arguments
    /// `real_id`: Real id of the mapping.
    ///
    /// # Returns
    /// Virtual id of id: `real_id`.
    pub fn virt_id_of(&self, real_id: usize) -> usize {
        self.real_to_virt.get(&real_id).copied().unwrap()
    }

    /// # Arguments
    /// `virt_id`: Virtual id of the mapping.
    ///
    /// # Returns
    /// Real id of id: `virt_id`.
    pub fn real_id_of(&self, virt_id: usize) -> usize {
        self.virt_to_real.get(&virt_id).copied().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        // Given: An empty id map.
        let id_map = IdMap::init(0);

        // When: Doing nothing.

        // Then:
        assert!(id_map.virt_to_real.is_empty());
        assert!(id_map.real_to_virt.is_empty());
    }

    #[test]
    fn put_real_to_virt() {
        // Given: An empty id map.
        let mut id_map = IdMap::init(1);

        // When: Adding a mapping from 1(real) to 2(virtual)
        id_map.put_real_to_virt(1, 2);

        // Then:
        assert_eq!(*id_map.real_to_virt.get(&1).unwrap(), 2);
        assert_eq!(id_map.virt_to_real.len(), 0);
    }

    #[test]
    fn put_virt_to_real() {
        // Given: An empty id map.
        let mut id_map = IdMap::init(1);

        // When: Adding a mapping from 1(virtual) to 2(real)
        id_map.put_virt_to_real(0, 1);

        // Then:
        assert_eq!(*id_map.virt_to_real.get(&0).unwrap(), 1);
        assert!(id_map.real_to_virt.is_empty());
    }

    #[test]
    fn get_real_to_virt() {
        // Given: An empty id map.
        let mut id_map = IdMap::init(1);

        // When: Adding a mapping from 1(real) to 2(virtual).
        id_map.put_real_to_virt(0, 1);

        // Then: Api must return the mapping.
        assert_eq!(id_map.virt_id_of(0), 1);
    }

    #[test]
    fn get_virt_to_real() {
        // Given: An empty id map.
        let mut id_map = IdMap::init(1);

        // When: Adding a mapping between from 1(virtual) to 2(real).
        id_map.put_virt_to_real(0, 1);

        // Then: Api must return the mapping.
        assert_eq!(id_map.real_id_of(0), 1);
    }
}
