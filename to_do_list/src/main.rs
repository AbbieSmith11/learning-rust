// Import standard library's input / output
use std::io;

// This is used for debugging later by printing the struct using {:?}
#[derive(Debug)]
struct Task {
    // A struct is a way to define your own custom data type (similar to js object)
    description: String,
    done: bool,
}

// An enum lets you define a set of possible named values (variants)
enum Command {
    Add,
    List,
    Complete,
    Quit,
    Unknown, // for anything unrecognised
}

fn main() {
    // a vector (Vec<T>) is like a growable list or array
    // This vector stores "Task" struct values
    let mut tasks: Vec<Task> = Vec::new();

    println!("Rust terminal to do app!");
    println!("Type: add, list, complete, or quit");

    // A loop that repeats forever until break is used to break out of it
    loop {
        println!("\n What would you like to do? (add, list, complete or quit)");

        // A new empty string to hold user input
        let mut input = String::new();

        // Reads a line of text from the user and stores it in input
        // stdin() gives access to keyboard input
        // read_line(&mut input) takes a *mutable reference* to the string. A reference is like pointing to the origional value without copying it
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read input"); // if fails expect() will crash the program with the message

        // Convert the user's input to lower case so theres no issues with caps cause string unmatch
        // trim() removes trailing new line characters ("\n")
        let cleaned = input.trim().to_lowercase();

        // Convert the cleaned string into one of the command enum options
        // Match runs through each pattern and picks the first one that matches
        let command = match cleaned.as_str() {
            "add" => Command::Add,
            "list" => Command::List,
            "complete" => Command::Complete,
            "quit" => Command::Quit,
            _ => Command::Unknown,
        };

        // This handles the command above using match again
        match command {
            Command::Add => {
                println!("Enter a new task:");

                // A string to store the task the user types
                let mut desc = String::new();

                // Read the task description
                io::stdin()
                    .read_line(&mut desc)
                    .expect("Failed to read task");

                // Creat a new "Task" struct instance
                // Fill in the fields of the struct here
                let task = Task {
                    description: desc.trim().to_string(),
                    done: false,
                };

                // Push the task into the vector
                tasks.push(task);

                println!("Task added!")
            }

            Command::List => {
                println!("\n Your to-do list:");

                if tasks.is_empty() {
                    println!("No tasks yet!")
                }

                // iter() looks at each task without taking ownership of it
                // enumerate() gives us both (index, item)
                for (i, task) in tasks.iter().enumerate() {
                    // An inline if expression
                    let status = if task.done { "✅" } else { "  " };

                    println!("{}: [{}] {}", i, status, task.description);
                }
            }

            Command::Complete => {
                println!("Enter the task number to mark as complete:");

                let mut num = String::new();
                io::stdin()
                    .read_line(&mut num)
                    .expect("Failed to read number");

                // Try to convert the string into an unsigned integer
                let parsed = num.trim().parse::<usize>();

                match parsed {
                    // Ok(index) means the parse sucseeded
                    // if index < tasks.len() ensures it is not out of range
                    Ok(index) if index < tasks.len() => {
                        // Accessing tasks[index] mutably to change it's "done" field
                        tasks[index].done = true;
                        println!("task marked as complete!");
                    }

                    // Anything else is invalid
                    _ => {
                        println!("Invalid task number");
                    }
                }
            }

            Command::Quit => {
                println!("Goodbye!");
                break; // break exits the loop and ends the program
            }

            Command::Unknown => {
                println!("Unknown command. Try: add, list, complete or quit");
            }
        }
    }
}
