fn main() {
    let mut v1 = vec!["Alice".to_string(); 3];
    let mut v2 = vec!["Bob".to_string(); 3];
    v1.append(&mut v2);
    println!("v1: {:?}", v1);
    println!("v2: {:?}", v2);
    println!("v1[3]: {}", v1.get(3).unwrap());

    v1.retain(|element| if element == "Alice" { true } else { false });
    println!("v1: {:?}", v1);
    println!("v1 len: {}", v1.len());
    println!("v1 capacity: {}", v1.capacity());

    v1.reserve(1000);
    println!("v1 len: {}", v1.len());
    println!("v1 capacity: {}", v1.capacity());
}
