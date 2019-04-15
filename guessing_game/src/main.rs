use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    game_loop(secret_number);
}

fn game_loop(secret_number: u32) {
    loop {
        println!("Guess the number!");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");
        if guess.trim() == "q" {
            break;
        }
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a digit!");
                continue;
            }
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Try higher"),
            Ordering::Greater => println!("Try lower"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
