use std::{collections::HashMap, sync::LazyLock, time::Instant};
use deepsize::DeepSizeOf;
use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng, Rng};

pub fn run() {
    let size = Size {
        keys: 2_000_000,
        hits: 2_000_000,
        misses: 2_000_000
    };

    run_hash_map(size);
}

pub struct Size {
    keys: usize,
    hits: usize,
    misses: usize
}

pub fn run_hash_map(size: Size) {
    let (map, duration, memory) = measure(|| build_hash_map(size.keys));
    println!("HashMap build: {:?} | memory: {} MB", duration, memory / 1_000_000);
    assert!(!map.is_empty());

    let (found, time, mem) = measure(|| measure_key_checks(|key| map.contains_key(key), size.hits, size.misses));
    println!("Found: {}, Time: {:?}, mem: {}", found, time, mem);
}

#[allow(dead_code)]
pub fn build_hash_map(username_count: usize) -> HashMap<String, usize> {
    let mut map = HashMap::new();
    let mut generator = UsernameGenerator::new();
    for _ in 0..username_count {
        let username = generator.next();
        *map.entry(username).or_insert(0) += 1;
    }
    map
}

#[allow(dead_code)]
fn measure_key_checks<F: Fn(&String) -> bool>(
    contains_key: F,
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
    let (found, time, _) = measure(|| {
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

static USERNAMES: LazyLock<Vec<String>> = LazyLock::new(|| {
    std::fs::read_to_string("tests/data/names.txt")
        .expect("Unable to read file")
        .trim()
        .split("\n")
        .map(|s| s.to_string())
        .collect()
});

fn measure<F, T>(executable: F) -> (T, std::time::Duration, usize)
where 
    F: FnOnce() -> T,
    T: DeepSizeOf
{
    let start = Instant::now();
    let map = executable();
    let duration = start.elapsed();
    let memory = map.deep_size_of();
    (map, duration, memory)
}
