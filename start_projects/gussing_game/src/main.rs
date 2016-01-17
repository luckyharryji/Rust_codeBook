
// author by xiangyu on 1-17-2016
extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){
	println!("Guess the number!");

	let secret_number = rand::thread_rng().gen_range(1,101);

	println!("The secreate number is {}", secret_number);

	loop {
		println!("Please Input your Guess:");

		let mut guess = String::new();
		io::stdin().read_line(&mut guess)
			.expect("Failed to load");

		let guess: u32 = match guess.trim().parse(){
			Ok(value_of_guess) => value_of_guess,
			Err(_) => continue,
			// .expect("Please type a number !");
		};

		println!("Your Guess: {}",guess);

		match guess.cmp(&secret_number){
			Ordering::Less => println!("Too small!"),
			Ordering::Greater => println!("Too big!"),
			Ordering::Equal => {
				println!("You Win !!");
				break;
			}
		}
	}
}