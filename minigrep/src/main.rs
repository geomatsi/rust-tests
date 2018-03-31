use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("config error: {}", err);
        process::exit(1);
    });
    println!("config '{:?}'", config);

    let mut fh = File::open(config.fname).unwrap_or_else(|err| {
        println!("file open error: {}", err);
        process::exit(1);
    });

    let mut text = String::new();
    fh.read_to_string(&mut text).unwrap_or_else(|err| {
        println!("read file error: {}", err);
        process::exit(1);
    });
}

#[derive(Debug)]
struct Config {
    query: String,
    fname: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let q = &args[1];
        let f = &args[2];

        Ok(Config {
            fname: f.clone(),
            query: q.clone(),
        })
    }
}
