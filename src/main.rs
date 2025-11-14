use library::is_leap;
use std::io;

fn main() {

    // leap year kata
    println!("Check to see if a specific year is a leap year, what year would you like to check?");

    let year = loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse::<i32>() {
            Ok(num) => break num,
            Err(_) => {
                println!("Invalid input. Please enter a valid number:");
                continue;
            }
        }
    };


    is_leap(year);

}
