use std::time::Instant;

use deepsize::DeepSizeOf;
use num_format::{Locale, ToFormattedString};
use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng, Rng};

use crate::{util::USERNAMES, Size};

pub fn run_benchmark<T: Set<String> + DeepSizeOf>(size: &Size, mut map: T, name: &str) {
    println!("============================================== {} ==============================================", name);
    let ((total_element_size, map_size), duration) = measure(|| build_set(&mut map, size.keys));
    println!(
        "{} build duration: {:?} | total element size: {} bytes | hash map size: {} bytes",
        name, 
        duration,
        total_element_size.en(),
        map_size.en()
    );
    assert!(!map.empty());

    let (found, time) = measure(|| measure_key_checks(|key| map.contains(key), size.hits, size.misses));
    println!("{}", found);
    println!(
        "Expected hits: {}, Actual hits: {}, False Positve Percentage: {}, Time: {:?}",
        size.hits.en(),
        found.en(),
        format!("{}%", (found - size.hits) / size.hits),
        time
    );
    println!("");
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

fn get_hit<T: Rng>(rng: &mut T) -> String {
    USERNAMES.choose(rng).unwrap().clone() + "_0"
}

fn get_miss<T: Rng>(rng: &mut T) -> String {
    (0..10).map(|_| rng.gen_range('a'..='z')).collect()
}

pub trait Set<K> {
    fn add_key(&mut self, key: K);
    
    fn contains(&mut self, key: &K) -> bool;

    fn empty(&mut self) -> bool;
}

pub trait Fmt<T> {
    fn en(&self) -> String;
}

impl<T: ToFormattedString> Fmt<T> for T {
    fn en(&self) -> String {
        self.to_formatted_string(&Locale::en)
    }
}

//===============================================

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
