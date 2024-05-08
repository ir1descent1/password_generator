use std::env;
use std::collections::HashMap;
use rand::Rng;

fn print_help() {
    println!("PASSWORD GENERATOR\n\
    Generate a password of a certain length with certain character types.\n\n\
    Usage:\npassgen [OPTION]... [LENGTH]\n\nOptions:\n\
    \t-l\tlowercase in password\n\
    \t-u\tuppercase in password\n\
    \t-n\tnumeric in password\n\
    \t-s\tsymbolic in password\n")
}

// Generates a random character based on the option passed
fn get_rand_char(inp: &String) -> char {
    let mut result: char = ' ';

    if inp == "uppercase" {
        result = char::from_u32(rand::thread_rng().gen_range(65..91)).unwrap();
    } else if inp == "lowercase" {
        result = char::from_u32(rand::thread_rng().gen_range(97..123)).unwrap();
    } else if inp == "number" {
        result = char::from_u32(rand::thread_rng().gen_range(48..58)).unwrap();
    } else if inp == "symbol" {
        let unscaled = rand::thread_rng().gen_range(33..65);

        // Accounts for the four different regions of symbols
        if unscaled >= 48 && unscaled < 55 {
            result = char::from_u32(unscaled + 10).unwrap();
        } else if unscaled >= 55 && unscaled < 61 {
            result = char::from_u32(unscaled + 36).unwrap();
        } else if unscaled >= 61 && unscaled < 65 {
            result = char::from_u32(unscaled + 62).unwrap();
        } else {
            result = char::from_u32(unscaled).unwrap();
        }
    }

    result
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Syntax error handling
    if args.len() != 3 || &args[1][..1] != "-" || !args[2].chars().all(char::is_numeric) {
        print_help();
        std::process::exit(0);
    }

    // Gets values from user input
    let options: String = String::from(&args[1][1..]);
    let length: usize = args[2].trim().parse().unwrap();

    // Mutable variables for recording values
    let mut count: i8 = 1;
    let mut result: String = String::new();
    let mut rand_to_char: HashMap<i8, String> = HashMap::new();

    // Populates hashmap based on tags used
    if options.contains("u") {
        rand_to_char.insert(*&count, String::from("uppercase"));
        count += 1;
    }
    if options.contains("l") {
        rand_to_char.insert(*&count, String::from("lowercase"));
        count += 1;
    }
    if options.contains("n") {
        rand_to_char.insert(*&count, String::from("number"));
        count += 1;
    }
    if options.contains("s") {
        rand_to_char.insert(*&count, String::from("symbol"));
        count += 1;
    }

    // Generates a random number corresponding to a character type, and then gets a random character
    // of that type. Finally, the character is appended to the result.
    let mut rng = rand::thread_rng();
    while result.len() < length {
        let char_type= rand_to_char.get(&rng.gen_range(1..*&count)).unwrap();
        let rand_char: char = get_rand_char(char_type);
        result += &*rand_char.to_string();
    }

    println!("{}", result);
}
