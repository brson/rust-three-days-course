use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, true);
    println!("{:?}", map.get(&1));
    println!("{:?}", map.contains_key(&1));
    
    for (k, v) in map {
        println!("{:?}: {:?}", k, v)
    }
}