use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is {}", secret_number);

    loop {
        println!("Input number");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Need a number");
                continue;
            }
        };

        println!("You guess: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("YES");
                break;
            }
            Ordering::Greater => println!("BIG"),
            Ordering::Less => println!("SMALL"),
        }
    }
}
