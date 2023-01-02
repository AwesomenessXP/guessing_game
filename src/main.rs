use std::io; // std library, '::' means an associated function of String type
use rand::Rng;
use std::cmp::Ordering; // Ordering is an enum with variants Less, Greater, Equal

// notes: 'cargo update' finds latest versions for your dependencies

fn main() {
    println!("Guess the number!");

    let secret_number = 
        rand::thread_rng() // gives us some random number
        .gen_range(1..=100); // get the range of numbers

    println!("The secret number is: {secret_number}");

    loop { // loop is an infinite loop!
        println!("Please input your guess");
        // mut makes variable mutable!
        let mut guess = String::new(); // creates a new, mutable, empty string
    
        io::stdin() // handles user input
            .read_line(&mut guess) // gets input from user, & means reference
            .expect("Failed to read line"); // error handling
    
        // guess is redefined: this is called shadowing
        // trim() removes whitespaces
        // parse() converts string to another type
        // let guess: u32 = guess.trim().parse().expect("Please enter a number");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, // arm returns Ok value with result number
            Err(_) => continue, // arm underscore means catch all values
        }; // we replace expect with  parse which returns a Result enum with variants: Ok and Err
    
        println!("You guessed: {}", guess);
    
        // match expression is made of "arms"
        // "arms" have a pattern to match against
        match guess.cmp(&secret_number){ // switch statement
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
