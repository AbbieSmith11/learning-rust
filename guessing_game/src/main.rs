// Guessing game that generates a random integer between 1 and 100.
// It will then prompt the player to enter a guess.
// After the guess is entered, the program will indicate wether the guess is too high or too low.
// If the guess is correct, the game will print a congratulatory message and exit.

// Importing io (input/output library) from the standard library known as std.
use std::io;

// Rng trait defines methods that random number generators implement.
use rand::Rng;

// Importing from the standard library: the Ordering type is another enum and has variants less, greater and equal.
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    // rand::thread_rng function gives a particular random number generator: one that is local to the current thread of execution and is seeded by the operating system.
    // Then we call the gen_range method on the random number generator. This method is defined by the Rng trait that we brought into scope with the use rand::Rng; statement.
    // The gen_range method takes a range expression as an argument and generates a random number in the range.
    // This kind of range expression takes the form start..=end and is inclusive on the lower and upper bounds, so it's needed to specify 1..=100 to request a number between 1 and 100.
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // The loop keyword creates an infinite loop
    loop {
        println!("Please input your guess");

        // Variables are unmutable by default so mut is needed to make a variable mutable
        // String::new is a function that returns a new instance of a string. string is a string type provided by the standard library that is a growable, UTF-8 encoded bit of text.
        // The :: syntax in the ::new indicates that new is an associated function of the string type. An associated function is a function that's implemented on a type. In this case string. This new function creates a new empty string.
        // Summary: this line has created a mutable variable that is currently bound to a new, empty instance of a string.
        let mut guess = String::new();

        // Calling the stdin function from the io module, which will allow us to handle user input.
        io::stdin()
            // This line calls the read_line method on the standard input handle to get input from the user.
            // We're also passing &mut guess as the argument to read_line to tell it what string to store the user input in.
            // The full job of read_line is to take whatever the user types into standard input and append that into a string (without overwriting its contents), so we therefore pass that string as an argument. The string argument needs to be mutable so the method can change the string's content.
            // the & indicates that this argument is a reference , which gives you a way to let multiple parts of your code access one peice of data without needing to copy that data inpot memory multiple times.
            // References (like variables) are ummutable by default so mut is needed to make it mutable
            .read_line(&mut guess)
            // Result is an enum which is a type that can be one of multiple states . Each possible state is called a variant.
            // Result's variants are ok and Err. Ok = success and contains successfully generated value. Err = failue and contains info on how or why it failed
            .expect("faied to read line");

        // Rust allows us to shadow the previous value of guess with a new one.
        // Shadowing lets us reuse the guess variable name rather than forcing us to create two unique variables such as guess_str and guess.
        // Shadowing is often used when you want to convert a value from one type to another type
        // This variable is binded to the expression guess.trim().parse(). The guess in the expression refers to the origional guess variable that contained input as a string.
        // The trim method on a string will eliminate any whitespace at the beginning and end.
        // The parse method on strings converts a string to another type.
        // This uses a match expression, described on line 82.
        // The underscore is a catch all value, effectvely telling program to ignore all errors that parse might encounter.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // This line prints the string that is the user's input.
        // This is for printing the value of a variable, but if you want to print the result of an expression, do this : println!("text {}", y + 2);
        println!("You guessed: {guess}");

        // The cmp method compares two values and can be called on anything that can be compared.
        // It then returns a variant of the ordering enum.
        // The match expression is used to decide what to do next based on which variant was returned.
        // A match expression is made up of arms. Rust takes the value given to match and looks through each arm pattern in turn and looks for a match.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
