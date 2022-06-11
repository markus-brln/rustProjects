// add export RUST_BACKTRACE=1 to ~/.profile to get a stack trace

mod sections {
    mod primitives;
    pub use primitives::*;
}
pub use sections::*; // use all fn from sections


fn main() {
    // Primitives
    // begin()
    // debug()
    // display()
    // primitive_types()
    // tuples()
    // arrays()

 
}
