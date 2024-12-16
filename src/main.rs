use std::io::stdin;

use rand::{thread_rng, Rng};

fn main() {
    let the_number = thread_rng().gen_range(1..=100);

    println!("I'm thinking of a number between 1 and 100. Take a guess:");

    'input: loop {
        for i in 1..=10 {
            let mut input = String::new();

            stdin().read_line(&mut input).unwrap();

            if let Ok(guess) = input.trim().parse::<i32>() {
                if guess >= 1 && guess <= 100 {
                    match guess.cmp(&the_number) {
                        std::cmp::Ordering::Equal => {
                            println!("Yay, you guessed the number!");
                            break 'input;
                        }
                        std::cmp::Ordering::Greater => {
                            println!("Your guess is too high.");
                        }
                        std::cmp::Ordering::Less => {
                            println!("Your guess is too low.");
                        }
                    }
                    if guess != the_number && i == 10 {
                        println!("Game over. The number I was thinking of was {}", the_number);
                        break 'input;
                    }
                } else {
                    println!("Please enter a positive whole number from 1 through 100.");
                    continue 'input;
                }
            } else {
                println!("Please enter a positive whole number from 1 through 100.");
                continue 'input;
            }
        }
    }
}
