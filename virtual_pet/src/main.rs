use std::io;

#[derive(Debug)]
struct Pet {
    // struct that defines what data the pet has
    name: String,
    hunger: i32,    // 0 = full, higher = hungrier
    happiness: i32, //higher = happier
    energy: i32,    // higher = more energetic
}

// this creates an implementation block where we can define functions (called METHODS) that belong specifically to the Pet type
impl Pet {
    // A function to make a new pet with default stats
    fn new(name: String) -> Self {
        Self {
            name,
            hunger: 0,
            happiness: 10,
            energy: 10,
        }
    }

    // Function to feed the pet
    fn feed(&mut self) {
        println!("\nYou fed {}!", self.name); // self refers to the struct being implemented
        self.hunger -= 3; // hunger gets better
        if self.hunger < 0 {
            self.hunger = 0; // keep in range
        }

        self.happiness += 1;
        println!("{} looks happy!\n", self.name);
    }

    // Function to play wih pet
    fn play(&mut self) {
        if self.energy <= 2 {
            println!("\n {} is too tired to play!", self.name);
            return;
        }
        println!("you played with {}!\n", self.name);
        self.happiness += 3;
        self.energy -= 3; // Playing uses energy
        self.hunger += 2; // Playing makes the pet hungry
    }

    // Let pet sleep
    fn sleep(&mut self) {
        println!("\n{} is taking a nap...", self.name);
        self.energy += 5; // Regains energy
        self.hunger += 2; // but wakes up hungry
        println!("{}, wakes up refreshed!\n", self.name);
    }

    // Show the pet's current stats
    fn show_stats(&self) {
        println!("\n PET STATUS");
        println!("Name: {}", self.name);
        println!("Hunger: {}", self.hunger);
        println!("Happiness: {}", self.happiness);
        println!("Energy: {}", self.energy);
    }

    // Each "turn" the pet naturally changes
    fn passive_update(&mut self) {
        self.hunger += 1; // gets slowly hungrier
        self.energy -= 1; // slowly gets tired
        self.happiness -= 1; // gets lonely without attention
    }

    // Print message if pet is unhappy
    fn emotional_reaction(&self) {
        // &self means to borrow the Pet but not able to modify it as doesnt have word mut
        if self.hunger >= 10 {
            println!("\n{} is starving!!!\n", self.name);
        }
        if self.energy <= 0 {
            println!("\n{} is getting exhausted!!!\n", self.name);
        }
        if self.happiness <= 0 {
            println!("\n{} feels lonely!!!\n", self.name);
        }
    }
}

fn main() {
    println!("Welcome to your virtual pet!");
    println!("What would you like to name your pet?\n");

    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read name");

    // trim() removes invisible newline characters like "\n"
    // to_string() converts the trimmed &str to a full string
    let name = name.trim().to_string();

    let mut pet = Pet::new(name);

    println!("\nYour pet {} is born", pet.name);

    pet.show_stats();

    // creates and infinite loop
    loop {
        println!("\n-------------------------");
        println!("\nWhat woud you like to do?");
        println!("1) Feed");
        println!("2) Play");
        println!("3) Sleep");
        println!("4) Check Stats");
        println!("5) Quit");
        println!("\n-------------------------\n");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");

        // match is rust's version of a js switch statement
        match choice.trim() {
            "1" => pet.feed(),
            "2" => pet.play(),
            "3" => pet.sleep(),
            "4" => pet.show_stats(),
            "5" => {
                println!("\n{} says goodbye!\n", pet.name);
                break;
            }
            _ => println!("Invalid choice! Try again."),
        }

        pet.passive_update();

        pet.emotional_reaction();
    }
}
