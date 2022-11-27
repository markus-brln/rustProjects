use threadpool::{Builder};
use std::sync::{Arc};
use std::sync::atomic::{AtomicUsize, Ordering};


fn main() {
    // create at least as many workers as jobs or you will deadlock yourself
    let n_workers = 12;
    let n_jobs = 20;
    let pool = Builder::new()
        .num_threads(n_workers)
        .thread_stack_size(8_000_000)
        .build();
    let an_atomic = Arc::new(AtomicUsize::new(0));

    for job in 0..n_jobs {
        // let barrier = barrier.clone();
        let an_atomic = an_atomic.clone();

        pool.execute(move|| {
            // do the heavy work
            an_atomic.fetch_add(1, Ordering::Relaxed);
            let mut a = 0;
            for _ in 0..100_000_000 {
                a += (a + 1) % 22;
            }
            println!("Hello job {job:?}, an_atomic {an_atomic:?}, a {a}");
        });
    }
    pool.join();

    assert_eq!(an_atomic.load(Ordering::SeqCst), n_jobs);
    println!("Hello, world!");
}
