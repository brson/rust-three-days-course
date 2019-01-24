// &mut denotes a mutable borrow.
// There is also &, for immutable borrow.
fn accepts_borrow(thing: &mut u32) {
    *thing += 1
}

fn main() {
    let mut value = 1;
    accepts_borrow(&mut value);
    println!("{}", value)
}