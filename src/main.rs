#[cfg(feature = "dhat_profiler")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

#[cfg(feature = "jemalloc")]
#[global_allocator]
static ALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

fn main() {
    #[cfg(feature = "dhat_profiler")]
    let _profiler = dhat::Profiler::new_heap();

    bloom_filter::run();
}
