use colored::*;
use std::io;

fn main() {
    println!("Temperature Converter!");
    println!("1. Fahrenheit to Celsius.");
    println!("2. Celsius to Fahrenheit.");

    let mut choice: String = String::new();

    println!("Enter your choice (1 or 2):");

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line.");

    let choice: u8 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Invalid choice input.");
            return;
        }
    };

    match choice {
        1 => {
            let mut temp_f: String = String::new();
            println!("Enter Tempert in Fahrenheit.");

            io::stdin()
                .read_line(&mut temp_f)
                .expect("Failed to read line.");
        }
        2 => {}
        _ => eprintln!("Invalid choice. Please enter 1 or 2."),
    }
}
