use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string_pretty};

#[derive(Serialize, Deserialize, Debug)]
struct Dog {
    name: String,
    year_born: i32,
    owner: DogOwner,
}

#[derive(Serialize, Deserialize, Debug)]
struct DogOwner {
    first_name: String,
    last_name: String,
}

fn main() {
    let dog01 = Dog {
        name: "Alice".to_string(),
        year_born: 2024,
        owner: DogOwner {
            first_name: "John".to_string(),
            last_name: "Smith".to_string(),
        },
    };

    let dog_ser = to_string_pretty(&dog01);
    if dog_ser.is_ok() {
        println!("{}", dog_ser.ok().unwrap());
    } else {
        println!("{:#?}", dog_ser.err());
    }

    let json_string = r#"
    {
      "name": "Alice",
      "year_born": 2024,
      "owner": {
        "first_name": "John",
        "last_name": "Smith"
      },
      "breed": "Great Pyrenees"
    }
    "#;

    let dog_deser = from_str::<Dog>(json_string);
    if dog_deser.is_ok() {
        println!("{:#?}", dog_deser.ok().unwrap());
    } else {
        println!("{:#?}", dog_deser.err());
    }
}
