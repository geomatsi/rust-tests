extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
	println!("guess the number:");

	let secret = rand::thread_rng().gen_range(1, 101);
	println!("secret: {}", secret);

	loop {
		let mut guess = String::new();
		io::stdin().read_line(&mut guess).expect("failed to read line");

		let guess: u32 = guess.trim().parse().expect("need a number...");
		println!("guess: {}", guess);

		match guess.cmp(&secret) {
			Ordering::Less => println!("too small..."),
			Ordering::Greater => println!("too big..."),
			Ordering::Equal => {
				println!("nice!");
				break;
			}
		}
	}
}
