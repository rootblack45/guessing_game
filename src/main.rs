use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the number from 1 to 10!");

    loop {
        println!("==================================");
        println!("Please input your guess: ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        match guess.trim().parse::<u8>() {
            Ok(num) => {
                println!("You guessed: {}", num);

                let secret_number: u8 = rand::thread_rng().gen_range(1..10);
                match num.cmp(&secret_number) {
                    Ordering::Less => println!("Too small!"),
                    Ordering::Greater => println!("Too big!"),
                    Ordering::Equal => {
                        println!("You win!");
                        break;
                    }
                }
                println!("The secret number is: {}", secret_number);
            }
            Err(_) => println!("Please input a number!"),
        }
    }
}
