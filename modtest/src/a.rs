// public
pub fn a_pub() -> String {
	a_priv();

	String::from("a pub")
}

// private
fn a_priv() {
	println!("Test a priv");
}
