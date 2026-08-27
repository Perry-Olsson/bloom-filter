use bloom_filter::run;

#[cfg(feature = "dhat_profiler")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    #[cfg(feature = "dhat_profiler")]
    let _profiler = dhat::Profiler::new_heap();

    run();
}
