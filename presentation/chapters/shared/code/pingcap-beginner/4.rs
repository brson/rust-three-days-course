enum Option {
    Some(u64),
    None,
}

fn main() {
    let value = Option::None;
    let other = Option::Some(1_u64);
}