use std::default::Default;

#[derive(Debug)]
struct FirstName(String);
impl Default for FirstName {
    fn default() -> Self {
        FirstName("@_@".to_string())
    }
}

//#[derive(Default)]
#[derive(Debug)]
struct Person {
    first_name: FirstName,
    last_name: String,
    age: u8,
    location: String,
}

impl Default for Person {
    fn default() -> Self {
        Self {
            first_name: FirstName::default(),
            last_name: "Smith".to_string(),
            age: 20,
            location: "Louisiana".to_string(),
        }
    }
}

impl Person {
    fn new() -> Self {
        Self {
            first_name: FirstName("".to_string()),
            last_name: "".to_string(),
            age: 0,
            location: "".to_string(),
        }
    }
}

fn main() {
    let person01 = Person {
        first_name: FirstName("John".to_string()),
        last_name: "Smith".to_string(),
        age: 20,
        location: "Louisiana".to_string(),
    };
    println!("{:?}", person01);

    let person02 = Person::new();
    println!("{:?}", person02);

    let person03 = Person::default();
    println!("{:?}", person03);
}
