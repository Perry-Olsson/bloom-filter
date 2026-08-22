use std::{collections::HashMap, sync::LazyLock, time::Instant};
use deepsize::DeepSizeOf;
use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng, Rng};

#[test]
fn test_hash_map() {
    let (map, duration, memory) = measure(|| build_hash_map(2000));
    println!("HashMap build: {:?} | memory: {} bytes", duration, memory);
    assert!(!map.is_empty());

    let hits = 10;
    let misses = 10;
    let (found, query_duration, _) = measure(|| query_hash_map(&map, hits, misses));
    println!(
        "HashMap query: {} found ({} hit attempts + {} misses) | duration: {:?}",
        found,
        hits,
        misses,
        query_duration
    );
}

fn build_hash_map(username_count: usize) -> HashMap<String, usize> {
    let mut map = HashMap::new();
    let mut generator = UsernameGenerator::new();
    for _ in 0..username_count {
        let username = generator.next();
        *map.entry(username).or_insert(0) += 1;
    }
    map
}

trait StringSet {
    fn contains_key(&self, key: &str) -> bool;
}

impl StringSet for HashMap<String, usize> {
    fn contains_key(&self, key: &str) -> bool {
        self.contains_key(key)
    }
}

fn query_hash_map<T: StringSet>(
    map: &T,
    hits: usize,
    misses: usize,
) -> usize {
    let mut rng = thread_rng();
    let mut hits_count = hits;
    let mut misses_count = misses;

    let mut found = 0;
    while hits_count > 0 || misses_count > 0 {
        let val: f64 = rng.gen_range(0.0..=1.0);
        let get_value: fn(&mut ThreadRng) -> String;
        if hits_count != 0 && (val > 0.5 || misses_count == 0) {
            get_value = get_hit;
            hits_count -= 1;
        } else  {
            get_value = get_miss;
            misses_count -= 1;
        }

        let val = &get_value(&mut rng);
        if map.contains_key(val) {
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
