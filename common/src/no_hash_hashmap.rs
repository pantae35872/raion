use std::{
    collections::HashMap,
    hash::{BuildHasherDefault, Hasher},
};

#[derive(Default)]
pub struct NoOpHasher {
    finish: u64,
}

impl Hasher for NoOpHasher {
    fn write(&mut self, _bytes: &[u8]) {}
    fn write_u64(&mut self, value: u64) {
        self.finish = value;
    }

    fn finish(&self) -> u64 {
        self.finish
    }
}

pub type NoHashHashMap<K, V> = HashMap<K, V, BuildHasherDefault<NoOpHasher>>;
