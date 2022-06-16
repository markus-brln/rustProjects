// create:
// rustc --crate-type=lib lib.rs


fn main() {
    lib::public_function();

    // Error! `private_function` is private
    //lib::private_function();

    lib::indirect_access();
}
