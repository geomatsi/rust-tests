use std::env;
use std::process;
use std::fs::File;
use std::error::Error;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("make config error: {}", err);
        process::exit(1);
    });
    println!("config '{:?}'", config);

    if let Err(e) = run(config) {
        println!("run config error: {}", e);
        process::exit(1);
    };
}

fn run(config: Config) -> Result<(), Box<Error>> {
    let mut fh = File::open(config.fname)?;
    let mut text = String::new();

    fh.read_to_string(&mut text)?;

    Ok(())
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
