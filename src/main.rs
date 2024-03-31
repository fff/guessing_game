
use std::io;

use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number between (0,100)!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut round = 0;

    loop {
        let mut guess = String::new();
        println!("Please input your guess. Round:{round}");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                match round.cmp(&5)  {
                    Ordering::Less => println!("Nice! You win in less than 5 rounds."),
                    _ => println!("You win! After {round} rounds guessing.")
                }
                break;
            }
        }

        round+=1;
    }
}
