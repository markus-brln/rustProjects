// create:
// rustc --crate-type=lib mylib.rs

// use:
// rustc main.rs --extern mylib=libmylib.rlib --edition=2018 && ./main
fn main() {
    mylib::public_function();

    // Error! `private_function` is private
    //lib::private_function();

    mylib::indirect_access();
}
