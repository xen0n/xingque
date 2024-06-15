use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub(crate) trait TrivialPyHash {
    fn trivial_py_hash(&self) -> u64;
}

impl<T: Hash> TrivialPyHash for T {
    fn trivial_py_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}
