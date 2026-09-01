use std::collections::HashMap;

use bloom_filter;

#[test]
fn test_hash_map() {
    let size = 10;
    let mut map = HashMap::new();
    bloom_filter::build_set(&mut map, size);
    assert_eq!(size, map.len())
}
