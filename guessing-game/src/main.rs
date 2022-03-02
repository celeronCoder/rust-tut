use std::{io::stdin, cmp::Ordering};
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    
    loop {
        println!("Please input your guess: ");
        
        let mut guess: String = String::new();

        stdin().read_line(&mut guess).expect("Failed to read line.");

        // Rust allows us to shadow the previous value of guess with a new one.
        let guess: u32 = guess.trim().parse().expect("Please type a number only");

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                return;
            },
        }
    }
}
