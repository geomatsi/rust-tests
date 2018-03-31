use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let (query, fname) = parse_config(&args);
    println!("file '{}' query '{}'", query, fname);
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let q = &args[1];
    let f = &args[2];

    return (f, q);
}

//let mut fh = File::open(fname).expect("file not found");
//let mut text = String::new();
//fh.read_to_string(&mut text).expect("failed to read from file");
