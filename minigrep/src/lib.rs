use std::fmt;
use std::fs::File;
use std::error::Error;
use std::io::prelude::*;

pub struct Config {
    pub query: String,
    pub fname: String,
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.fname, self.query)
    }
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();    // skip arg[0]

        let q = match args.next() {
            Some(arg) => arg,
            None => return Err("Missing query string..."),
        };

        let f = match args.next() {
            Some(arg) => arg,
            None => return Err("Missing filename..."),
        };

        Ok(Config {
            fname: f,
            query: q,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut fh = File::open(config.fname)?;
    let mut text = String::new();

    fh.read_to_string(&mut text)?;

    for line in search(&config.query, &text) {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, data: &'a str) -> Vec<&'a str> {
    data.lines().filter(|line| line.contains(query)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "not enough arguments")]
    fn test_config_err1() {
        Config::new(&[]).unwrap();
    }

    #[test]
    #[should_panic(expected = "not enough arguments")]
    fn test_config_err2() {
        let a = String::from("a");

        Config::new(&[a]).unwrap();
    }

    #[test]
    #[should_panic(expected = "not enough arguments")]
    fn test_config_err3() {
        let a = String::from("a");
        let b = String::from("b");

        Config::new(&[a, b]).unwrap();
    }

    #[test]
    fn  test_config_ok1() {
        let a = String::from("a");
        let b = String::from("b");
        let c = String::from("c");

        let t = Config::new(&[a, b, c]).unwrap();

        assert_eq!(t.query, "b");
        assert_eq!(t.fname, "c");
    }

    #[test]
    fn test_search1() {
        let q = "a";
        let t = "b"; 

        assert_eq!(0, search(q, t).len());
    }

    #[test]
    fn test_search2() {
        let q = "a";
        let t = "a b c"; 

        assert_eq!(vec!["a b c"], search(q, t));
    }

    #[test]
    fn test_search3() {
        let q = "a";
        let t = "a
                 b c"; 

        assert_eq!(vec!["a"], search(q, t));
    }
}
