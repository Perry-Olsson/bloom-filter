use std::{collections::HashMap, sync::LazyLock};

#[test]
fn test_hash_map() {
    let map = build_hash_map(2000);
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
