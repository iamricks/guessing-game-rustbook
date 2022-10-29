use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    print_separator();
    println!("Guess the number!");
    println!("Type 'quit' to exit at any time.");
    print_separator();
    println!("\n");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    loop {
        let guess = get_guess();
    
        if should_quit(&guess) {
            break;
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That is not a number!");
                continue
            },
        };
    
        if check_guess(&guess, &secret_number) {
            break;
        }
    }
}

fn get_guess() -> String {
    println!("Please input your guess.");

    let mut guess = String::new();
    
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    return guess;
}

// Quit the program if the user types "quit"
fn should_quit(guess: &String) -> bool {
    guess.trim() == "quit"
}

// Check if the guess is correct and return true if it is
fn check_guess(guess: &u32, secret_number: &u32) -> bool {
    println!("You guessed: {guess}");
    
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
            return true;
        }
    }

    return false;
}

fn print_separator() {
    println!("====================");
}