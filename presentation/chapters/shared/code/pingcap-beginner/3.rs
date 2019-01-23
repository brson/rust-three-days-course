struct Empty;

struct WithFields {
    foo: i32,
    bar: bool,
}

fn main() {
    let value = WithFields {
        foo: 1,
        bar: false,
    };
    println!("{}", value.foo);
}