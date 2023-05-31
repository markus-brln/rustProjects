use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use image;
use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage};
use num_complex::Complex;


fn read_file() {
    // Create a path to the desired file
    let path = Path::new("assets/hello.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file: File = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", &path.display(), why),
        Ok(file) => file,
    };

    print!("{:?}", file);

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }

    // `file` goes out of scope, and the "hello.txt" file gets closed
}


fn read_image(path: &str) {
    let img = match image::open(path) {
        Err(why) => panic!("couldn't open {}: {}", path, why),
        Ok(file) => file,
    };
    let dimensions = img.dimensions();
    println!("dimensions {:?}", dimensions);
    println!("{:?}", img.color());
    // print!("{:?}", img);
    let imgx = dimensions.0;
    let imgy = dimensions.1;

    let scalex = 3.0 / imgx as f32;
    let scaley = 3.0 / imgy as f32;

    // for x in 0..imgx {
    //     for y in 0..imgy {
    //         let cx = y as f32 * scalex - 1.5;
    //         let cy = x as f32 * scaley - 1.5;
    //
    //         let c = num_complex::Complex::new(-0.4, 0.6);
    //         let mut z = num_complex::Complex::new(cx, cy);
    //
    //         let mut i = 0;
    //         while i < 255 && z.norm() <= 2.0 {
    //             z = z * z + c;
    //             i += 1;
    //         }
    //
    //         let pixel = img.get_pixel(x, y);
    //         // println!("{:?}", pixel);
    //
    //         let image::Rgb([i, 0, 0, 1]);
    //         // *pixel = image::Rgb([data[0], i as u8, data[2], 1]);
    //     }
    // }

    for (x, y, mut pixel) in img.pixels() {
        // print!("{:?}", pixel)
        pixel[0] = 0;
    }

    img.save("fractal.png").unwrap();
}


fn main() {
    read_file();
    read_image("assets/image.png")
}