fn main() {
    let v = vec![1,2,3];
    let i = v.iter();
    let i2 = double(i);
}

fn double<'a>(i: impl Iterator<Item=&'a u8> + 'a) -> impl Iterator<Item=u8> + 'a {
    i.map(|v| (*v)*2)
}