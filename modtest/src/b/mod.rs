// public
pub fn b_pub() -> String {
    b_priv();

    String::from("b pub")
}

// private
fn b_priv() {
    println!("Test b priv");
}
