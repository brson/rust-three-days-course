use std::fs::File;

fn main() -> Result<()> {
    // Open a file, call the network, etc.
    let result = File::open("/some_file");
    // This is either `Ok(the_file)`
    // Or an error.

    // You can use the `?` operator to safely pass the error up.
    // Like an exception*** (Which we don't have)
    let file_handle = result?;

    // Or, unsafely call `unwrap()` to crash if there is an error.
    let file_handle = result.unwrap();
}