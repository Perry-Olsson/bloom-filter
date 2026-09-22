pub mod util;

use std::{collections::{HashMap, HashSet}, hash::{DefaultHasher, Hash, Hasher}};
use deepsize::DeepSizeOf;

use crate::util::benchmarks::{run_benchmark, Set};

pub fn run(size: Size) {
    run_hash_map(&size);
    run_finger_print_hash(&size);
}

pub fn run_finger_print_hash(size: &Size) {
    let map = FingerPrintHash::new();
    run_benchmark(size, map, "FingerPrintHash");
}

pub fn run_hash_map(size: &Size) {
    let map: HashMap<String, usize> = HashMap::new();
    run_benchmark(size, map, "HashMap");
}

pub struct Size {
    pub keys: usize,
    pub hits: usize,
    pub misses: usize
}

impl<K: Eq + Hash> Set<K> for HashMap<K, usize> {
    fn add_key(&mut self, key: K) {
        self.entry(key).or_insert(1);
    }

    fn contains(&mut self, key: &K) -> bool {
        self.contains_key(key)
    }

    fn empty(&mut self) -> bool {
        self.is_empty()
    }
}

struct FingerPrintHash {
    finger_prints: HashSet<u64>
}

impl FingerPrintHash {
    fn new() -> FingerPrintHash {
        FingerPrintHash {
            finger_prints: HashSet::new()
        }
    }

    fn contains_key<K: Eq + Hash>(&mut self, key: &K) -> bool {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        self.finger_prints.contains(&hasher.finish())
    }

    fn is_empty(&self) -> bool {
        self.finger_prints.is_empty()
    }
}

impl DeepSizeOf for FingerPrintHash {
    fn deep_size_of_children(&self, _: &mut deepsize::Context) -> usize {
        self.finger_prints.deep_size_of()
    }
}

impl<K: Eq + Hash> Set<K> for FingerPrintHash {
    fn add_key(&mut self, key: K) {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        self.finger_prints.insert(hasher.finish());
    }

    fn contains(&mut self, key: &K) -> bool {
        self.contains_key(key)
    }

    fn empty(&mut self) -> bool {
        self.is_empty()
    }
}
