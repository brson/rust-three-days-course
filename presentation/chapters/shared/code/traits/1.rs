fn main() {
    let v = vec![1,2,3];
    let i = make_iter(&v);
}

fn make_iter<'a>(v: &'a Vec<u8>) -> impl Iterator<Item=u8> + 'a {
    v.iter().map(|v| (*v)*2)
}