extern crate rand;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    

    let secret = rand::thread_rng().gen_range(1, 10);

    println!("The secret number is {}", secret);
    loop {
        println!("Enter your guess");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num, 
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small :("),
            Ordering::Greater => println!("Too big :("),
            Ordering::Equal => {
                println!("You got it! :D");
                break;
            }
        }
    }
}
