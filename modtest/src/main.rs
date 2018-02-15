extern crate modtest;

use modtest::a;

fn main() {
	println!("Hello modules");

	let ax = a::a_pub();
	let bx = modtest::b::b_pub();

	println!("a -> {}", ax);
	println!("b -> {}", bx);
}
