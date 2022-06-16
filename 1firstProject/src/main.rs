// add export RUST_BACKTRACE=1 to ~/.profile to get a stack trace

mod sections {
    mod primitives;
    pub use primitives::*;
    mod higher_order_fun;
    pub use higher_order_fun::*;
}
pub use sections::*; // use all fn from sections
use std::time::Instant;

fn main() {
    let upper = 100000000000;

    // Imperative approach
    let now = Instant::now();
    let mut acc = 0_u64;
    for n in 0.. {
        let n_squared = n * n;

        if n_squared >= upper {
            break;
        } else if is_odd(n_squared) {
            acc += n_squared;
        }
    }
    let elapsed_time = now.elapsed();

    println!("imperative style took {:?} microseconds \n{}", elapsed_time.as_micros(), acc);
    // Primitives
    // begin()
    // debug()
    // display()
    // primitive_types()
    // tuples()
    // arrays()
    // higher_order()

 
}
