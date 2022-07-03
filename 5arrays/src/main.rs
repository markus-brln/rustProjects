#![allow(dead_code)]

use std::env;
use ndarray::*;
extern crate rand;
use rand::{thread_rng, Rng};


fn env_basics() {
    println!("{:?}", env::current_dir());
}


fn array_basics() {
    let mut a2 = Array2::<f64>::zeros((3, 3));
    a2[[0, 0]] = 0.0;
    a2[[0, 1]] = 0.5;
    a2[[0, 2]] = -0.5;
    a2[[1, 0]] = 1.0;
    a2[[1, 1]] = 1.5;
    a2[[1, 2]] = -1.5;
    a2[[2, 0]] = 2.0;
    a2[[2, 1]] = 2.5;
    a2[[2, 2]] = -2.5;
    println!("The 2D array is {:?}", a2);

    let arr = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let mut a2 = arr2(&arr);
    a2[[2, 1]] = 10;
    println!("The 2D array is {:?}", a2);

    for row in a2.genrows() {
        // Each row is a 1D array
        println!("row is {:?}", row);
    }

    let a3 = arr3(&[[[1, 2], [3, 4]],
        [[5, 6], [7, 8]],
        [[9, 0], [1, 2]]]);

    for a2 in a3.outer_iter() {
        println!("2D array is {:?}", a2);
    }

    let array_from_other = array!(&[1, 2]);
    println!("array_from_other is {:?}", array_from_other);
}


fn random_basics() {
    let mut arr = [0u64; 4];
    
    // thread_rng().try_fill(&mut a3);
    let res = thread_rng().try_fill(&mut arr[..]);
    println!("Random number array {:?} {:?}", arr, res);

    
    let mut rng = thread_rng();
    let n: f32 = rng.gen_range(0.0..10.0);
    println!("{}", n);
}

fn main() {
    // env_basics();
    // array_basics();
    random_basics();

}