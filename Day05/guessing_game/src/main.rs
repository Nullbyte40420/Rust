use std::io; // this line imports the io module from the standard library
use rand::Rng;// this line imports the Rng trait from the rand crate
use std::cmp::Ordering;// this line imports the Ordering enum from the std::cmp module

fn main(){
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); // this line generates a random number between 1 and 100 (inclusive) and binds it to the variable secret_number

    loop{
        println!("Please input your guess.");
        let mut guess = String::new(); // line has created a mutable variable that is currently bound to a new, empty instance of a String

        // if don't write use std::io; then we have to write std::io::stdin() instead of just stdin()
        io::stdin() // calling stdin() fundtion from the io module
            .read_line(&mut guess) // this method takes a mutable reference to the String variable guess and reads a line of input from the standard input (stdin) and appends it to the String variable guess
            .expect("Failed to read input");

        let guess: u32 = match guess.trim().parse() { // this line takes the String variable guess, trims any whitespace from the beginning and end of the string, and then attempts to parse it as an unsigned 32-bit integer (u32)
            Ok(num) => num, // this line will return the value of num if the parse() method is successful
            Err(_) => continue, // this line will continue the loop if the parse() method is unsuccessful
        };

        match guess.cmp(&secret_number) { // this line compares the value of guess to the value of secret_number and returns an Ordering enum (Less, Greater, or Equal)
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
    }
    }
}
