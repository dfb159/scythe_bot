use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let guess = get_guess();

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Your guess of {guess} is too small!"),
            std::cmp::Ordering::Equal => {
                println!("You win! The secret number was indeed {secret_number}!");
                break;
            }
            std::cmp::Ordering::Greater => println!("Your guess of {guess} is too big!"),
        }
    }
}

fn get_guess() -> i32 {
    println!("Please input your guess:");
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line!");

        let guess_parsed = match guess.trim().parse::<i32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Not an integer number. Try again:");
                continue;
            }
        };

        if guess_parsed < 1 || guess_parsed > 100 {
            println!("The number is out of range. Try again:");
            continue;
        }

        return guess_parsed;
    }
}
