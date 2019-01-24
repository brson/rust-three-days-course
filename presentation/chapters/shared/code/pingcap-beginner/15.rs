mod foo {
    pub fn do_a_thing() {
        // ...
    }
}

// Lets you just call `do_a_thing()` instead of
// `foo::do_a_thing()`
use foo::do_a_thing;

// Will try to open `./bar.rs` relative to this file.
pub mod bar;

fn main() {
    do_a_thing()
}