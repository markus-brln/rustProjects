use threadpool::ThreadPool;
use std::sync::{Arc, Barrier};
use std::sync::atomic::{AtomicUsize, Ordering};


fn main() {
    // create at least as many workers as jobs or you will deadlock yourself
    let n_workers = 42;
    let n_jobs = 99;
    let pool = ThreadPool::new(n_workers);
    let an_atomic = Arc::new(AtomicUsize::new(0));

    assert!(n_jobs <= n_workers, "too many jobs, will deadlock");

    // create a barrier that waits for all jobs plus the starter thread
    let barrier = Arc::new(Barrier::new(n_jobs + 1));
    for job in 0..n_jobs {
        let barrier = barrier.clone();
        let an_atomic = an_atomic.clone();


        pool.execute(move|| {
            // do the heavy work
            an_atomic.fetch_add(1, Ordering::Relaxed);
            println!("Hello job {job:?}, barrier {barrier:?}, an_atomic {an_atomic:?}");
            // then wait for the other threads
            barrier.wait();
        });
    }

    // wait for the threads to finish the work
    barrier.wait();
    assert_eq!(an_atomic.load(Ordering::SeqCst), /* n_jobs = */ 23);
    println!("Hello, world!");
}
