/**
  * The following code has been written according to the instructions
  * from the Rust docs.
  * Nevertheless I learned a lot from it and want to share this simple
  * guessing game with the people ;-)
  * ~ David Brandstetter
  *
  * To run, just execute the complied file in /target/release/guessing-game
**/

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
	println!("Guess the number! It is between 1 and 10 :-D");

	let secret_number = rand::thread_rng().gen_range(1..=10);

	// println!("The secret number is {secret_number}");

	loop {
		println!("Please input your guess.");

		let mut guess = String::new();

		io::stdin()
			.read_line(&mut guess)
			.expect("Failed to read line");

		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => {
				println!("Expected a number, not {guess}");
				continue;
			}
		};

		println!("You guessed: {guess}");

		match guess.cmp(&secret_number) {
			Ordering::Less => println!("Too small!"),
			Ordering::Greater => println!("Too big!"),
			Ordering::Equal => {
				println!("You win, Rustacean!");
				break;
			},
		}
	}
}