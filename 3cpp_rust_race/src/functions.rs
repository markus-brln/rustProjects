#![allow(dead_code)]


pub fn simple_loop() -> u64 {
    let mut sum = 0_u64;
    for _idx in 0..1000000000 {
        sum += 1;
    }

    sum
}


pub fn loop_add_idx() -> u64 {
    let mut sum = 0_u64;

    let mut idx = 0_u64;
    loop {
        if idx == 10000000000 {
            break;
        }
        sum += idx % 3;    
        idx += 1;    
    }

    sum
}


pub fn vec() -> u32{
    let mut vec: Vec<i32> = Vec::new();
    
    for idx in 0..100000 {
        vec.push(idx % 3)
    }
    // println!("{:?}", vec);
    return 0;
}


pub fn vec_2d() -> u32{
    let mut vec: Vec<Vec<i16>> = Vec::new();
    
    for _idx in 0..1000 {
        // let mut v: Vec<i16> = Vec::with_capacity(1000);
        let mut v: Vec<i16> = Vec::new();
        for i in 0..1000 {
            v.push(i);
        }
        // println!("{}", v.len());
        vec.push(v);
    }


    // println!("{:#?}", vec);
    return 0;
}
