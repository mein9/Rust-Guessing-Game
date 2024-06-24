use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main() {
    println!("You have 10 tries to guess the number!");

    let random_number: u32 = rand::thread_rng().gen_range(1..=100);

    let mut counter = 0;

    loop {
        if counter == 11 {
            println!("You are out of guesses!");
            break;
        }

        println!("Please input your guess.");

        let mut guess: String = String::new();
        counter += 1;

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");
    

        match guess.cmp(&random_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }

        }
    }
}