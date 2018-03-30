use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let query = &args[1];
    let fname = &args[2];

    println!("grep for {} in file {}", query, fname);
}
