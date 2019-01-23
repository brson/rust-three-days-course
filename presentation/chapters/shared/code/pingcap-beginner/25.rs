struct Point<Precision> {
    x: Precision,
    y: Precision
}

fn main() {
    let point: Point<u8> = Point { x: 1, y: 2 };
    let point: Point<i32> = Point { x: 1, y: 2 };
}
