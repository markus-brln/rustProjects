#![allow(dead_code)] // An attribute to hide warnings for unused code.

use ferris_says::say;
use std::io::{stdout, BufWriter};


pub fn begin() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();

    let x = 5 + /* 90 + */ 5;
    println!("Is `x` 10 or 100? x = {}", x);

    let number: f64 = 1.0;
    let offset: usize = 6;
    println!("{number:>offset$}");

    println!(
        "{} of {:b} people know binary, the other half doesn't",
        1, 2
    );
}

pub fn debug() {
    #[derive(Debug)]
    struct DebugPrintable(i32);

    #[derive(Debug)]
    struct Deep(DebugPrintable);
    let my_obj: DebugPrintable = DebugPrintable(2);
    println!("{:#?}", my_obj);
    println!("{:#?}", Deep(my_obj));

    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8,
    }
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Pretty print
    println!("{:#?}", peter);
}

// Display
use std::fmt;
struct Structure(i32);

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub fn display() {
    let hello: u128 = 354435436546345879798743535;
    println!("{hello}");

    println!("{}", Structure(22));
}

pub fn primitive_types() {
    // Variables can be type annotated.
    let logical: bool = true;

    let a_float: f64 = 1.0; // Regular annotation
    let an_integer = 5i32; // Suffix annotation

    // Or a default will be used.
    let default_float = 3.0; // `f64`
    let default_integer = 7; // `i32`

    // A type can also be inferred from context
    let mut inferred_type = 12; // Type i64 is inferred from another line
    println!("{inferred_type}");
    inferred_type = 4294967296i64;

    // A mutable variable's value can be changed.
    let mut mutable = 12; // Mutable `i32`
    println!("{mutable}");
    mutable = 21;
    println!("{mutable}");

    // Error! The type of a variable can't be changed.
    // mutable = true;

    // Variables can be overwritten with shadowing.
    let mutable = true;

    println!(
        "{} {} {} {} {} {} {}",
        logical, a_float, an_integer, default_float, default_integer, inferred_type, mutable
    )
}

// Tuples
pub fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (integer, boolean) = pair;
    (boolean, integer)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

pub fn tuples() {
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
        -1i8, -2i16, -3i32, -4i64,
        0.1f32, 0.2f64,
        'a', true);

    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);

    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    let pair = (1, true);
    println!("pair is {:?}", pair);

    println!("the reversed pair is {:?}", reverse(pair));
    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));

    let tuple = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);
}

// Arrays
use std::mem;

pub fn analyze_slice(slice: &[i32]) { // This function borrows a slice
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

pub fn arrays() {
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let ys: [i32; 500] = [0; 500];

    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);
    println!("number of elements in array: {}", xs.len());
    println!("array occupies {} bytes", mem::size_of_val(&xs));    // Arrays are stack allocated
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);
    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1 .. 4]);
    // println!("{}", xs[5]); // idx error runtime
}