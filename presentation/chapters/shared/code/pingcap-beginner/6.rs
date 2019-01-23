fn main() {
    let maybe_value = Some(2);
    
    if let Some(value) = maybe_value {
        // `value` is bound to the contents, 2.
        println!("{}", value);
    } else { /* ... */ }
}