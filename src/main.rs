use library::is_leap;
use std::io;

fn main() {

    // leap year kata
    println!("Check to see if a specific year is a leap year, what year would you like to check?");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    is_leap(input.trim().parse().expect("Please enter a valid number"));

}
