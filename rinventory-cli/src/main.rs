use clap::{App, load_yaml};
use rinventory_core::{log};

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from(yaml).get_matches();

    if let Some(_) = matches.value_of("dryrun") {
        println!("Dryrun is set!")
    }

    match matches.occurrences_of("v") {
        0 => println!("Verbose mode is off"),
        1 => println!("Verbose mode is kind of on"),
        2 => println!("Verbose mode is on"),
        3 | _ => println!("Don't be crazy"),
    }

    if let Some(ref matches) = matches.subcommand_matches("create") {
        if let Some(ref matches) = matches.subcommand_matches("product") {
            println!("We are in the product!");
            if let Some(id) = matches.value_of("id") {
                println!("The id is {}", id);
            }
            if let Some(name) = matches.value_of("name") {
                println!("The name is {}", name);
            }
        } else {
            println!("Invalid subcommand")
        }
    }

    log("Hello there");
}
