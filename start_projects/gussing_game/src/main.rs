
use std::io;

fn main(){
	println!("Guess the number!");
	println!("Please Input your Guess:");

	let mut guess = String::new();
	io::stdin().read_line(&mut guess)
		.expect("Failed to load");

	println!("Your Guess: {}",guess);
}