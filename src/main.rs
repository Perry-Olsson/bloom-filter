use bloom_filter::Size;

#[cfg(feature = "dhat_profiler")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

#[cfg(feature = "jemalloc")]
#[global_allocator]
static ALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

fn main() {
    #[cfg(feature = "dhat_profiler")]
    let _profiler = dhat::Profiler::new_heap();

    let size = Size {
        keys: 20000,
        hits: 20000,
        misses: 20000 
    };

    bloom_filter::run(size);
}
