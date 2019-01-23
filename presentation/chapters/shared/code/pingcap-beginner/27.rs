fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    println!("{:?}", vec.pop());
    println!("{:?}", vec.pop()); // What does this return?
    
    for v in vec {
        println!("{:?}", v)
    }
}