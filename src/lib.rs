pub mod util;

use std::{collections::{HashMap, HashSet}, hash::{DefaultHasher, Hash, Hasher}, time::Instant};
use deepsize::DeepSizeOf;
use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng, Rng};

use util::USERNAMES;

pub fn run(size: Size) {
    run_hash_map(&size);
    run_finger_print_hash(&size);
}

pub struct Size {
    pub keys: usize,
    pub hits: usize,
    pub misses: usize
}

pub fn run_finger_print_hash(size: &Size) {
    let hasher = DefaultHasher::new();
    let map = FingerPrintHash::new(hasher);
    run_benchmark(size, map, "FingerPrintHash");
}

pub fn run_hash_map(size: &Size) {
    let map: HashMap<String, usize> = HashMap::new();
    run_benchmark(size, map, "HashMap");
}

pub fn run_benchmark<T: Set<String> + DeepSizeOf>(size: &Size, mut map: T, name: &str) {
    let ((total_element_size, map_size), duration) = measure(|| build_set(&mut map, size.keys));
    println!(
        "{} build duration: {:?} | total element size: {} | hash map size: {} bytes",
        name, 
        duration,
        total_element_size,
        map_size
    );
    assert!(!map.empty());

    let (found, time) = measure(|| measure_key_checks(|key| map.contains(key), size.hits, size.misses));
    println!("{}", found);
    println!(
        "Expected hits: {}, Actual hits: {}, False Positve Percentage: {}, Time: {:?}",
        size.hits,
        found,
        format!("{}%", (found - size.hits) / size.hits),
        time
    );
}

#[allow(dead_code)]
pub fn build_set<T: Set<String> + DeepSizeOf>(set: &mut T, username_count: usize) -> (usize, usize) {
    let mut generator = UsernameGenerator::new();
    let mut total_element_size = 0;
    for _ in 0..username_count {
        let username = generator.next();
        total_element_size += username.deep_size_of();
        set.add_key(username);
    }
    (total_element_size, (*set).deep_size_of())
}

#[allow(dead_code)]
fn measure_key_checks<F: FnMut(&String) -> bool>(
    mut contains_key: F,
    mut hits: usize,
    mut misses: usize,
) -> usize {
    let mut rng = thread_rng();
    let mut values = Vec::with_capacity(hits + misses);
    while hits > 0 || misses > 0 {
        let val: f64 = rng.gen_range(0.0..=1.0);
        let get_value: fn(&mut ThreadRng) -> String;
        if hits != 0 && (val > 0.5 || misses == 0) {
            get_value = get_hit;
            hits -= 1;
        } else  {
            get_value = get_miss;
            misses -= 1;
        }

        values.push(get_value(&mut rng));
    }
    let (found, time) = measure(|| {
        let mut found = 0;
        for val in &values {
            if contains_key(val) {
                found += 1;
            }
        }
        found
    });

    println!("Time spent on lookups: {:?}", time);
    found
}

#[allow(dead_code)]
fn query_hash_map<F: Fn(&String) -> bool>(
    contains_key: F,
    mut hits: usize,
    mut misses: usize,
) -> usize {
    let mut rng = thread_rng();
    let mut found = 0;
    while hits > 0 || misses > 0 {
        let val: f64 = rng.gen_range(0.0..=1.0);
        let get_value: fn(&mut ThreadRng) -> String;
        if hits != 0 && (val > 0.5 || misses == 0) {
            get_value = get_hit;
            hits -= 1;
        } else  {
            get_value = get_miss;
            misses -= 1;
        }

        let val = &get_value(&mut rng);
        if contains_key(val) {
            found += 1;
        }
    }
    found
}

fn get_hit<T: Rng>(rng: &mut T) -> String {
    USERNAMES.choose(rng).unwrap().clone() + "_0"
}

fn get_miss<T: Rng>(rng: &mut T) -> String {
    (0..10).map(|_| rng.gen_range('a'..='z')).collect()
}

struct UsernameGenerator {
    idx: usize,
    iteration: usize,
    usernames: Vec<String>
}

impl UsernameGenerator {
    fn new() -> Self {
        Self {
            idx: 0,
            iteration: 0,
            usernames: USERNAMES.clone()
        }
    }

    fn next(&mut self) -> String {
        let name = &self.usernames[self.idx];
        let appended_name = format!("{}_{}", name, self.iteration);
        self.idx += 1;
        if self.idx == self.usernames.len() {
            self.idx = 0;
            self.iteration += 1;
        }
        appended_name
    }
}

fn measure<F, T>(executable: F) -> (T, std::time::Duration)
where 
    F: FnOnce() -> T,
    T: DeepSizeOf
{
    let start = Instant::now();
    let map = executable();
    let duration = start.elapsed();
    (map, duration)
}

pub trait Set<K> {
    fn add_key(&mut self, key: K);
    
    fn contains(&mut self, key: &K) -> bool;

    fn empty(&mut self) -> bool;
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

struct FingerPrintHash<T: Hasher> {
    hasher: T,
    finger_prints: HashSet<u64>
}

impl<T: Hasher> FingerPrintHash<T> {
    fn new(hasher: T) -> FingerPrintHash<T> {
        FingerPrintHash {
            hasher,
            finger_prints: HashSet::new()
        }
    }

    fn contains_key<K: Eq + Hash>(&mut self, key: &K) -> bool {
        key.hash(&mut self.hasher);
        self.finger_prints.contains(&self.hasher.finish())
    }

    fn is_empty(&self) -> bool {
        self.finger_prints.is_empty()
    }
}

impl<T: Hasher> DeepSizeOf for FingerPrintHash<T> {
    fn deep_size_of_children(&self, _: &mut deepsize::Context) -> usize {
        self.finger_prints.deep_size_of() + 8
    }
}

impl<T: Hasher, K: Eq + Hash> Set<K> for FingerPrintHash<T> {
    fn add_key(&mut self, key: K) {
        key.hash(&mut self.hasher);
        self.finger_prints.insert(self.hasher.finish());
    }

    fn contains(&mut self, key: &K) -> bool {
        self.contains_key(key)
    }

    fn empty(&mut self) -> bool {
        self.is_empty()
    }
}
