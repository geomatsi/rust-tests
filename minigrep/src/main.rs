use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    // env example
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let query = &args[1];
    let fname = &args[2];

    println!("grep for {} in file {}", query, fname);

    // file io example
    let mut fh = File::open(fname).expect("file not found");
    let mut text = String::new();

    fh.read_to_string(&mut text).expect("failed to read from file");
    println!("Text:\n{}", text);
}
