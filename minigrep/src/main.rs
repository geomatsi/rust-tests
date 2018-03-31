use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let config = Config::new(&args);
    println!("config '{:?}'", config);

    let mut fh = File::open(config.fname).expect("file not found");
    let mut text = String::new();
    fh.read_to_string(&mut text)
        .expect("failed to read from file");
}

#[derive(Debug)]
struct Config {
    query: String,
    fname: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let q = &args[1];
        let f = &args[2];

        Config {
            fname: f.clone(),
            query: q.clone(),
        }
    }
}
