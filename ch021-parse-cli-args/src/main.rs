use std::env::args;

fn main() {
    let my_args: Vec<_> = args().collect();
    println!("{:?}", my_args);

    if my_args.len() != 3 {
        println!("You didn't specify two arguments");
        return;
    }

    // let name = my_args.iter().skip(1).take(1).next().unwrap();
    // let year_born = my_args.iter().skip(2).take(1).next().unwrap();
    let name: String = my_args.get(1).unwrap().into();
    let parsed_year = my_args.get(2).unwrap().parse::<i32>();
    if parsed_year.is_err() {
        println!("The specified dog year was invalid. Please specify an integer value");
        return;
    }
    let year_born = parsed_year.ok().unwrap();
    println!("{name} {year_born}");

    let dog01 = new_dog(name, year_born);
    dog01.get_details();
}

struct Dog {
    name: String,
    year_born: i32,
}

impl Dog {
    fn get_details(&self) {
        println!(
            "Dog name is {} and born in year {}",
            self.name, self.year_born
        );
    }
}

fn new_dog(name: String, year_born: i32) -> Dog {
    Dog { name, year_born }
}
