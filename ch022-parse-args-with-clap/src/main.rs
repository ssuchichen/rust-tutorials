use clap::{Arg, Command, command};

fn main() {
    let cmd = command!()
        .subcommand(
            Command::new("register-person")
                .arg(
                    Arg::new("firstname")
                        .long("first-name")
                        .short('f')
                        .aliases(&["fname", "firstname"])
                        .required(true)
                        .help("The person's firstname"),
                )
                .arg(
                    Arg::new("lastname")
                        .long("last-name")
                        .short('l')
                        .aliases(&["lname", "lastname"])
                        .required(true)
                        .help("The person's lastname"),
                ),
        )
        .subcommand(
            Command::new("register-pet").arg(
                Arg::new("pet-name")
                    .long("pet-name")
                    .short('n')
                    .required(true),
            ),
        )
        .about("This app registers people with their doctor's office.")
        .arg(
            Arg::new("fluffy")
                .long("fluffy")
                .help("Is the person wearing a fluffy coat or not"),
        )
        .get_matches();

    // println!("Fluffy: {}", cmd.get_one::<String>("fluffy").unwrap());
    // let pet_args = cmd.subcommand_matches("register-pet");
    // println!(
    //     "Does pet name exist ? {}",
    //     pet_args.unwrap().get_one::<String>("pet-name").unwrap()
    // );

    let person_args = cmd.subcommand_matches("register-person").unwrap();
    let firstname = person_args.get_one::<String>("firstname").unwrap();
    let lastname = person_args.get_one::<String>("lastname").unwrap();
    println!("First name: {}, lastname: {}", firstname, lastname);
}
