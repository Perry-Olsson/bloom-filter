use std::collections::HashMap;

use bloom_filter::{self, Size};

#[test]
fn test_hash_map() {
    let size = 10;
    let mut map = HashMap::new();
    bloom_filter::build_set(&mut map, size);
    assert_eq!(size, map.len())
}

#[test]
fn test_run() {
    let size = Size {
        keys: 20_000,
        hits: 20_000,
        misses: 20_000
    };

    bloom_filter::run(size);
}
