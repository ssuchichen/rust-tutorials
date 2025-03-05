use uuid::Uuid;

fn main() {
    let uuid = Uuid::new_v4();
    println!("Generated UUID is: {}", uuid.to_string());
}
