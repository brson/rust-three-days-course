enum Option<T> {
    None,
    Some(T),
}

fn main() {
    // A type annotation here is required,
    // Since we can't know what `Some(value)` might be.
    let alternative: Option<bool> = None;
}