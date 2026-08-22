use bloom_filter;

#[test]
fn test_hash_map() {
    let size = 10;
    let map = bloom_filter::build_hash_map(size);
    assert_eq!(size, map.len())
}
