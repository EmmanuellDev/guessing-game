use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        let mut guess = String::new();
        println!("Guess the number!");
        println!("Guess a number : ");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to load input data");
        let guess: u32 = guess.trim().parse().expect("Failed to parse");
        println!("You guessed {}", guess);
        println!("The actual generated number is {}", secret_number);
        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You guessed correctly!!");
                println!("Thanks for Playing");
                println!("bye bye!!");
                break;
            },
            Ordering::Less => println!("Your number is less than the actual number"),
            Ordering::Greater => println!("Your number is higher than the actual number"),
        }
    }
}
