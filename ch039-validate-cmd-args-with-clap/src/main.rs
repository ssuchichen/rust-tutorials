use clap::{Arg, Command, value_parser};
use regex::Regex;
use std::io::ErrorKind;

fn type_validator() {
    let coat_types = ["winter", "rain", "hoodie"];
    let coat_arg = Arg::new("coat-type")
        .long("coat-type")
        .value_parser(coat_types);
    let cmd = Command::new("base").arg(coat_arg);
    let result = cmd.get_matches();

    println!(
        "The coat type you passed in is {}",
        result.get_one::<String>("coat-type").unwrap()
    );
}

fn age_validator() {
    let age_arg = Arg::new("age")
        .long("age")
        .short('a')
        .help("This is the age that will be validated against.")
        .value_parser(value_parser!(u8).range(25..=40));
    let cmd = Command::new("base").arg(age_arg);
    let result = cmd.get_matches();

    println!("{}", result.get_one::<u8>("age").unwrap())
}

fn first_name() {
    let first_name_arg = Arg::new("first-name")
        .long("first-name")
        .help("Please enter your first name with a capital letter.")
        .value_parser(first_name_validator);
    let cmd = Command::new("base").arg(first_name_arg);
    let result = cmd.get_matches();
    println!(
        "The first name you entered is {}",
        result.get_one::<String>("first-name").unwrap()
    )
}

fn first_name_validator(value: &str) -> Result<String, std::io::Error> {
    let first_name_regex =
        Regex::new("[A-Z]\\w+").expect("You wrote an invalid regex. Please fix it.");
    if first_name_regex.is_match(value) {
        return Ok(value.to_string());
    }
    Err(std::io::Error::new(
        ErrorKind::Other,
        "Please make sure first name is capitalized.",
    ))
}

fn main() {
    // age_validator();
    // type_validator();
    first_name();
}
