// add export RUST_BACKTRACE=1 to ~/.profile to get a stack trace

mod sections {
    mod primitives;
    pub use primitives::*;
    mod higher_order_fun;
    pub use higher_order_fun::*;
    mod cfg;
    pub use cfg::*;
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
    higher_order()
    // cfg()
 
}
