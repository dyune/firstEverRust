use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Try to guess my number.");

    let target: i32 = rand::thread_rng().gen_range(1..=100);

    loop {
        // Declare `input` inside the loop to reset it each time
        let mut input = String::new();

        println!("Please put in your guess");

        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Trim and parse the input
        let guess: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            },
        };

        // Compare the guess to the target number
        match guess.cmp(&target) {
            Ordering::Less => println!("Too small..."),
            Ordering::Equal => {
                println!("Congrats, that's the one.");
                break;
            },
            Ordering::Greater => println!("Too big!"),
        }
    }
}
