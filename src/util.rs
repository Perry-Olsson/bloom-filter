use std::sync::LazyLock;

pub mod benchmarks;

pub static USERNAMES: LazyLock<Vec<String>> = LazyLock::new(|| {
    std::fs::read_to_string("tests/data/names.txt")
        .expect("Unable to read file")
        .trim()
        .split("\n")
        .map(|s| s.to_string())
        .collect()
});

