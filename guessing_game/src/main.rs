use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess that number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut first_run = true;

    loop {
        if first_run {
            println!("Please input your guess.");
        } else {
            println!("Please input your guess again.");
        }

        first_run = false;

        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        match guess.trim().to_lowercase().as_ref() {
            "quit" => break,
            _ => (),
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("Error on input: {}", err);
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
