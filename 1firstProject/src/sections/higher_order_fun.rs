#![allow(dead_code)]

use std::time::Instant;


pub fn is_odd(n: u64) -> bool {
    n % 2 == 1
}

pub fn higher_order() {
    println!("Find the sum of all the squared odd numbers under 1000");
    let upper = 100000000;

    // Imperative approach
    let mut now = Instant::now();
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

    println!("imperative style took {:?}", elapsed_time.as_secs_f64());
    println!("imperative style: {}", acc);

    // Functional approach
    now = Instant::now();
    let sum_of_squared_odd_numbers: u64 =
        (0..).map(|n| n * n)
             .take_while(|&n_squared| n_squared < upper)
             .filter(|&n_squared| is_odd(n_squared))
             .fold(0, |acc, n_squared| acc + n_squared);
    let elapsed_time = now.elapsed();
    println!("functional style took {:?}", elapsed_time.as_secs_f64());
    println!("functional style: {}", sum_of_squared_odd_numbers);
}