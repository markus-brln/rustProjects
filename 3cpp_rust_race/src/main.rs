use std::time::Instant;

mod functions;
pub use functions::*; // use all fn from functions


fn main() {
    let now = Instant::now();
    
    // let res = simple_loop(); // 70-150ns
    // let res = loop_add_idx(); // 8.67s
    // let res = vec(); // 375 µs
    let res = vec_2d(); // 0.003 s

    let elapsed_time = now.elapsed();
    println!("{:?}\n{:?}s", elapsed_time, elapsed_time.as_secs_f64());
    println!("Result: {:?}", res);
}
