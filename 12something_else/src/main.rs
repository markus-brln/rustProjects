use image;
use image::{GenericImageView, ImageBuffer, Rgb, Rgba};
use rand::Rng;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::time::Instant;

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


fn load_and_modify_copy(path: &str) {
    let img = match image::open(path) {
        Err(why) => panic!("couldn't open {}: {}", path, why),
        Ok(file) => file,
    };
    let (w, h) = img.dimensions();
    let mut output = ImageBuffer::new(w, h); // create a new buffer for our output

    for (x, y, pixel) in img.pixels() {
        let mut rng = rand::thread_rng();
        let random_float: f64 = rng.gen();  // float between 0 and 1
        let _random_float_in_range: f64 = rng.gen_range(0.0..10.0);

        if random_float > 0.5 {
            output.put_pixel(x, y, Rgba::<u8>([0, 0, 0, 255]));
        } else {
            output.put_pixel(x, y, pixel);
        }
    }
    let _ = output.save("modified.png");
}


fn make_fractal() {
    // 4 : 3 ratio is nice
    let width = 800;
    let height = 600;

    let mut img = ImageBuffer::new(width as u32, height as u32);

    // constants to tweak for appearance
    let cx = -0.9;
    let cy = 0.27015;
    let iterations = 110;

    for x in 0..width {
        for y in 0..height {
            let inner_height = height as f32;
            let inner_width = width as f32;
            let inner_y = y as f32;
            let inner_x = x as f32;

            let mut zx = 3.0 * (inner_x - 0.5 * inner_width) / (inner_width);
            let mut zy = 2.0 * (inner_y - 0.5 * inner_height) / (inner_height);

            let mut i = iterations;

            while zx * zx + zy * zy < 4.0 && i > 1 {
                let tmp = zx * zx - zy * zy + cx;
                zy = 2.0 * zx * zy + cy;
                zx = tmp;
                i -= 1;
            }

            // guesswork to make the rgb color values look okay
            let r = (i << 3) as u8;
            let g = (i << 5) as u8;
            let b = (i * 4) as u8;
            let pixel = Rgb([r, g, b]);
            img.put_pixel(x as u32, y as u32, pixel);
        }
    }

    let _ = img.save("output.png");
}


fn timeit<F: Fn() -> T, T>(function: F) -> T {
    // let start = SystemTime::now();
    let start = Instant::now();
    let result = function();
    // let end = SystemTime::now();
    // let duration = end.duration_since(start).unwrap();
    println!("it took {:?}", start.elapsed());
    result
}


fn main() {
    timeit(|| read_file());

    timeit(|| load_and_modify_copy("assets/image.png"));

    timeit(|| make_fractal());
}